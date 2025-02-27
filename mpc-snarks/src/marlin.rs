use super::silly::MySillyCircuit;
use ark_marlin::{ahp::prover::*, *};
use ark_poly::univariate::DensePolynomial;
use ark_poly_commit::marlin_pc::MarlinKZG10;
use ark_std::{end_timer, start_timer, test_rng};
use blake2::Blake2s;
use mpc_algebra::honest_but_curious::*;
use mpc_algebra::Reveal;

fn prover_message_publicize(
    p: ProverMsg<MpcField<ark_bls12_377::Fr>>,
) -> ProverMsg<ark_bls12_377::Fr> {
    match p {
        ProverMsg::EmptyMessage => ProverMsg::EmptyMessage,
        ProverMsg::FieldElements(d) => {
            ProverMsg::FieldElements(d.into_iter().map(|e| e.reveal()).collect())
        }
    }
}

fn comm_publicize(
    pf: ark_poly_commit::marlin_pc::Commitment<ME>,
) -> ark_poly_commit::marlin_pc::Commitment<E> {
    ark_poly_commit::marlin_pc::Commitment {
        comm: commit_from_mpc(pf.comm),
        shifted_comm: pf.shifted_comm.map(commit_from_mpc),
    }
}

fn commit_from_mpc<'a>(
    p: ark_poly_commit::kzg10::Commitment<MpcPairingEngine<ark_bls12_377::Bls12_377>>,
) -> ark_poly_commit::kzg10::Commitment<ark_bls12_377::Bls12_377> {
    ark_poly_commit::kzg10::Commitment(p.0.reveal())
}
fn pf_from_mpc<'a>(
    pf: ark_poly_commit::kzg10::Proof<MpcPairingEngine<ark_bls12_377::Bls12_377>>,
) -> ark_poly_commit::kzg10::Proof<ark_bls12_377::Bls12_377> {
    ark_poly_commit::kzg10::Proof {
        w: pf.w.reveal(),
        random_v: pf.random_v.map(MpcField::reveal),
    }
}

fn batch_pf_publicize(
    pf: ark_poly_commit::BatchLCProof<MFr, DensePolynomial<MFr>, MpcMarlinKZG10>,
) -> ark_poly_commit::BatchLCProof<Fr, DensePolynomial<Fr>, LocalMarlinKZG10> {
    ark_poly_commit::BatchLCProof {
        proof: pf.proof.into_iter().map(pf_from_mpc).collect(),
        evals: pf
            .evals
            .map(|e| e.into_iter().map(MpcField::reveal).collect()),
    }
}

pub fn pf_publicize(
    k: Proof<MpcField<ark_bls12_377::Fr>, MpcMarlinKZG10>,
) -> Proof<ark_bls12_377::Fr, LocalMarlinKZG10> {
    let pf_timer = start_timer!(|| "publicize proof");
    let r = Proof::<ark_bls12_377::Fr, LocalMarlinKZG10> {
        commitments: k
            .commitments
            .into_iter()
            .map(|cs| cs.into_iter().map(comm_publicize).collect())
            .collect(),
        evaluations: k.evaluations.into_iter().map(|e| e.reveal()).collect(),
        prover_messages: k
            .prover_messages
            .into_iter()
            .map(prover_message_publicize)
            .collect(),
        pc_proof: batch_pf_publicize(k.pc_proof),
    };
    end_timer!(pf_timer);
    r
}

type Fr = ark_bls12_377::Fr;
type E = ark_bls12_377::Bls12_377;
type ME = MpcPairingEngine<ark_bls12_377::Bls12_377>;
type MFr = MpcField<Fr>;
type MpcMarlinKZG10 = MarlinKZG10<ME, DensePolynomial<MFr>>;
type LocalMarlinKZG10 = MarlinKZG10<E, DensePolynomial<Fr>>;
type LocalMarlin = Marlin<Fr, LocalMarlinKZG10, Blake2s>;
type MpcMarlin = Marlin<MFr, MpcMarlinKZG10, Blake2s>;

pub fn mpc_test_prove_and_verify(n_iters: usize) {
    let rng = &mut test_rng();

    // First we create a setup for Marlin
    let srs = LocalMarlin::universal_setup(100, 50, 100, rng).unwrap();
    
    // Now we initialize an empty circuit - this is just so we can generate the circuit data - common to prover and verifier
    let empty_circuit: MySillyCircuit<Fr> = MySillyCircuit { a: None, b: None };
    let (index_pk, index_vk) = LocalMarlin::index(&srs, empty_circuit.clone()).unwrap();
    
    // We now create an MPC version of the prover index (since we will fill it with secret shared inputs)
    let mpc_index_pk = IndexProverKey::from_public(index_pk);

    for _ in 0..n_iters {
        // This creates "secret-shared" data s.t. the king holds 2 and the rest hold the identity element of the field
        let a = MpcField::<ark_bls12_377::Fr>::from(2u8);
        let b = MpcField::<ark_bls12_377::Fr>::from(2u8);
        
        // We then instantiate the circuit's private inputs with the secret-shared data
        let circ = MySillyCircuit {
            a: Some(a),
            b: Some(b),
        };

        // This runs the MPC to compute the result c = a•b
        let mut c = a;
        c *= &b;

        // Now the participants reveal their shares of the output - this should be public
        let inputs = vec![c.reveal()];
        println!("{}\n{}\n{}", a, b, c);

        // This should run the MPC version of Marlin for the circuit and generate a valid proof for regular Marlin
        // Essentially regular Marlin is run but over the MPC Field which abstracts away interaction between the parties when needed
        let mpc_proof = MpcMarlin::prove(&mpc_index_pk, circ, rng).unwrap();
        // We now "open" the proof to get the proof data in plaintext
        let proof = pf_publicize(mpc_proof);
        
        // We verify the proof with the normal Marlin verifier and it passes
        let is_valid = LocalMarlin::verify(&index_vk, &inputs, &proof, rng).unwrap();
        assert!(is_valid);

        // This is to show we really need to know c s.t. c = a•b and that o.w. verification fails
        let public_a = a.reveal();
        let is_valid = LocalMarlin::verify(&index_vk, &[public_a], &proof, rng).unwrap();
        assert!(!is_valid);
    }
}

pub fn mpc_demo_dragons() {
    let rng = &mut test_rng();

    // First we create a setup for Marlin
    let srs = LocalMarlin::universal_setup(100, 50, 100, rng).unwrap();
    
    // Now we initialize an empty circuit - this is just so we can generate the circuit data - common to prover and verifier
    let empty_circuit: MySillyCircuit<Fr> = MySillyCircuit { a: None, b: None };
    let (index_pk, index_vk) = LocalMarlin::index(&srs, empty_circuit.clone()).unwrap();
    
    // We now create an MPC version of the prover index (since we will fill it with secret shared inputs)
    let mpc_index_pk = IndexProverKey::from_public(index_pk);

    // This creates "secret-shared" data s.t. the king holds 2 and the rest hold the identity element of the field
    let a = MpcField::<ark_bls12_377::Fr>::from(2u8);
    let b = MpcField::<ark_bls12_377::Fr>::from(2u8);
    
    // We then instantiate the circuit's private inputs with the secret-shared data
    let circ = MySillyCircuit {
        a: Some(a),
        b: Some(b),
    };

    // This runs the MPC to compute the result c = a•b
    let mut c = a;
    c *= &b;

    // Now the participants reveal their shares of the output - this should be public
    let inputs = vec![c.reveal()];
    println!("{}\n{}\n{}", a, b, c);

    // This should run the MPC version of Marlin for the circuit and generate a valid proof for regular Marlin
    // Essentially regular Marlin is run but over the MPC Field which abstracts away interaction between the parties when needed
    let mpc_proof = MpcMarlin::prove(&mpc_index_pk, circ, rng).unwrap();
    // We now "open" the proof to get the proof data in plaintext
    let proof = pf_publicize(mpc_proof);
    
    // We verify the proof with the normal Marlin verifier and it passes
    let is_valid = LocalMarlin::verify(&index_vk, &inputs, &proof, rng).unwrap();
    assert!(is_valid);

    // This is to show we really need to know c s.t. c = a•b and that o.w. verification fails
    let public_a = a.reveal();
    let is_valid = LocalMarlin::verify(&index_vk, &[public_a], &proof, rng).unwrap();
    assert!(!is_valid);
}
