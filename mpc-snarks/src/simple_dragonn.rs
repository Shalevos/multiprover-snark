use ark_ff::Field;
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};

#[derive(Clone)]
pub struct SimpleDragonnCircuit<F: Field> {
    pub left_dragonn: Option<F>,
    pub right_dragonn: Option<F>,
}

impl<ConstraintF: Field> ConstraintSynthesizer<ConstraintF> for SimpleDragonnCircuit<ConstraintF> {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        let left_dragonn = cs.new_witness_variable(|| self.left_dragonn.ok_or(SynthesisError::AssignmentMissing))?;
        let right_dragonn = cs.new_witness_variable(|| self.right_dragonn.ok_or(SynthesisError::AssignmentMissing))?;
        let c = cs.new_input_variable(|| {
            let mut a = self.left_dragonn.ok_or(SynthesisError::AssignmentMissing)?;
            let b = self.right_dragonn.ok_or(SynthesisError::AssignmentMissing)?;

            a.mul_assign(&b);
            Ok(a)
        })?;

        cs.enforce_constraint(lc!() + left_dragonn, lc!() + right_dragonn, lc!() + c)?;

        Ok(())
    }
}
