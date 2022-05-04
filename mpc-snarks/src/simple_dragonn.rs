use ark_ff::Field;
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError, Variable},
};

#[derive(Clone)]
pub struct SimpleDragonnCircuit<F: Field> {
    pub l1: Option<F>,
    pub l2: Option<F>,
    pub r1: Option<F>,
    pub r2: Option<F>,
}

impl<ConstraintF: Field> ConstraintSynthesizer<ConstraintF> for SimpleDragonnCircuit<ConstraintF> {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        let l1 = cs.new_witness_variable(|| self.l1.ok_or(SynthesisError::AssignmentMissing))?;
        let l2 = cs.new_witness_variable(|| self.l2.ok_or(SynthesisError::AssignmentMissing))?;
        let r1 = cs.new_witness_variable(|| self.r1.ok_or(SynthesisError::AssignmentMissing))?;
        let r2 = cs.new_witness_variable(|| self.r2.ok_or(SynthesisError::AssignmentMissing))?;

        let v1 = cs.new_witness_variable(|| {
            let l1 = self.l1.ok_or(SynthesisError::AssignmentMissing)?;
            let r1 = self.r1.ok_or(SynthesisError::AssignmentMissing)?;
            let mut diff = l1 - r1;
            diff.square_in_place();
            Ok(diff)
        })?;

        let v2 = cs.new_witness_variable(|| {
            let l1 = self.l1.ok_or(SynthesisError::AssignmentMissing)?;
            let r1 = self.r1.ok_or(SynthesisError::AssignmentMissing)?;
            Ok(l1 * (ConstraintF::one() - r1))
        })?;

        let o1 = cs.new_witness_variable(|| {
            let l1 = self.l1.ok_or(SynthesisError::AssignmentMissing)?;
            let r1 = self.r1.ok_or(SynthesisError::AssignmentMissing)?;
            let mut diff = l1 - r1;
            diff.square_in_place();
            Ok(l1 * (ConstraintF::one() - r1) * diff)
        })?;

        let v3 = cs.new_witness_variable(|| {
            let l2 = self.l2.ok_or(SynthesisError::AssignmentMissing)?;
            let r2 = self.r2.ok_or(SynthesisError::AssignmentMissing)?;
            Ok(l2 * (ConstraintF::one() - r2))
        })?;

        let o2 = cs.new_witness_variable(|| {
            let l1 = self.l1.ok_or(SynthesisError::AssignmentMissing)?;
            let r1 = self.r1.ok_or(SynthesisError::AssignmentMissing)?;
            let l2 = self.l2.ok_or(SynthesisError::AssignmentMissing)?;
            let r2 = self.r2.ok_or(SynthesisError::AssignmentMissing)?;
            let mut diff = l1 - r1;
            diff.square_in_place();
            Ok((ConstraintF::one() - diff) * (l2 * (ConstraintF::one() - r2)))
        })?;

        let res = cs.new_input_variable(|| {
            let l1 = self.l1.ok_or(SynthesisError::AssignmentMissing)?;
            let r1 = self.r1.ok_or(SynthesisError::AssignmentMissing)?;
            let l2 = self.l2.ok_or(SynthesisError::AssignmentMissing)?;
            let r2 = self.r2.ok_or(SynthesisError::AssignmentMissing)?;
            let mut diff = l1 - r1;
            diff.square_in_place();
            Ok((ConstraintF::one() - diff) * (l2 * (ConstraintF::one() - r2)) + (l1 * (ConstraintF::one() - r1) * diff))
        })?;

        // cs.enforce_constraint(lc!() + l2, lc!() + (ConstraintF::from(2u8), l2) - (ConstraintF::from(4u8), Variable::One), lc!())?;

        // Verify is bit
        // For some reason adding this specific constraint a bajillion times just makes the program not crash
        // If we add less of these the spdz version breaks (the local version works) - WHY????
        cs.enforce_constraint(lc!() + l1, lc!(), lc!())?;
        cs.enforce_constraint(lc!() + l1, lc!(), lc!())?;
        cs.enforce_constraint(lc!() + l1, lc!(), lc!())?;
        cs.enforce_constraint(lc!() + l1, lc!(), lc!())?;
        cs.enforce_constraint(lc!() + l1, lc!(), lc!())?;
        cs.enforce_constraint(lc!() + l1, lc!(), lc!())?;
        cs.enforce_constraint(lc!() + l1, lc!(), lc!())?;
        cs.enforce_constraint(lc!() + l1, lc!(), lc!())?;
        cs.enforce_constraint(lc!() + l1, lc!(), lc!())?;
        cs.enforce_constraint(lc!() + l1, lc!(), lc!())?;
        cs.enforce_constraint(lc!() + l1, lc!(), lc!())?;
        cs.enforce_constraint(lc!() + l1, lc!(), lc!())?;
        // cs.enforce_constraint(lc!() + l1, lc!() + l2, lc!())?;
        // cs.enforce_constraint(lc!() + l1, lc!() + l2, lc!())?;
        // cs.enforce_constraint(lc!() + l1, lc!() + l2, lc!())?;
        // cs.enforce_constraint(lc!() + l1, lc!() + l2, lc!())?;
        // cs.enforce_constraint(lc!() + l1, lc!() + l2, lc!())?;
        // cs.enforce_constraint(lc!() + l1, lc!() + l2, lc!())?;
        // cs.enforce_constraint(lc!() + l1, lc!() + l2, lc!())?;
        // cs.enforce_constraint(lc!() + l1, lc!() + l2, lc!())?;
        // cs.enforce_constraint(lc!() + l1, lc!() + l2, lc!())?;
        // cs.enforce_constraint(lc!() + l1, lc!() + l2, lc!())?;

        cs.enforce_constraint(lc!() + l1, lc!() + l1 - Variable::One, lc!())?;
        cs.enforce_constraint(lc!() + l2, lc!() + l2 - Variable::One, lc!())?;
        cs.enforce_constraint(lc!() + r1, lc!() + r1 - Variable::One, lc!())?;
        cs.enforce_constraint(lc!() + r2, lc!() + r2 - Variable::One, lc!())?;
        // Could be removed since we check a public input value, but added for safety
        cs.enforce_constraint(lc!() + res, lc!() + res - Variable::One, lc!())?;
        // cs.enforce_constraint(lc!() + res, lc!() + res - Variable::One, lc!())?;

        
        // Compute xor(l_i, r_i)
        cs.enforce_constraint(lc!() + l1 - r1, lc!() + l1 - r1, lc!() + v1)?;
        // Compute l_i > r_i
        cs.enforce_constraint(lc!() + l1, lc!() + Variable::One - r1, lc!() + v2)?;
        // Compute l_1 > r_1 if they're different
        cs.enforce_constraint(lc!() + v1, lc!() + v2, lc!() + o1)?;
        // Compute l_i > r_i
        cs.enforce_constraint(lc!() + l2, lc!() + Variable::One - r2, lc!() + v3)?;
        // Compute l2 > r_2 if l1 == r2
        cs.enforce_constraint(lc!() + Variable::One - v1, lc!() + v3, lc!() + o2)?;
        // ORs both branches of the computation together s.t. res = l > r
        cs.enforce_constraint(lc!() + o1 + o2, lc!() + Variable::One, lc!() + res)?;

        Ok(())
    }
}
