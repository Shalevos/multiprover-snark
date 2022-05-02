use ark_ff::Field;
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError, Variable},
};

#[derive(Clone)]
pub struct SimpleDragonnCircuit<F: Field> {
    pub l1: Option<F>,
    pub l2: Option<F>,
    // pub r1: Option<F>,
    // pub r2: Option<F>,
}

impl<ConstraintF: Field> ConstraintSynthesizer<ConstraintF> for SimpleDragonnCircuit<ConstraintF> {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        let l1 = cs.new_witness_variable(|| self.l1.ok_or(SynthesisError::AssignmentMissing))?;
        let l2 = cs.new_witness_variable(|| self.l2.ok_or(SynthesisError::AssignmentMissing))?;
        // let r1 = cs.new_witness_variable(|| self.r1.ok_or(SynthesisError::AssignmentMissing))?;
        // let r2 = cs.new_witness_variable(|| self.r2.ok_or(SynthesisError::AssignmentMissing))?;

        // Verify is bit
        cs.enforce_constraint(lc!() + l1, lc!() + l1 - Variable::One, lc!())?;
        cs.enforce_constraint(lc!() + l1, lc!() + l1 - Variable::One, lc!())?;
        cs.enforce_constraint(lc!() + l2, lc!() + (ConstraintF::from(2u8), l2) - (ConstraintF::from(4u8), Variable::One), lc!())?;
        // cs.enforce_constraint(lc!() + r1, lc!() + r1 - Variable::One, lc!() + Variable::Zero)?;
        // cs.enforce_constraint(lc!() + r2, lc!() + r2 - Variable::One, lc!() + Variable::Zero)?;

        Ok(())
    }
}
