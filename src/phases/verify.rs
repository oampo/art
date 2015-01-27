use types::ArtResult;

use vm_inner::VMInner;
use expression::{Expression, ExpressionState};
use opcode::DspOpcode;

use instructions::dsp::unit::Unit;

pub trait Verify {
    fn verify(&mut self, expression: &mut Expression) -> ArtResult<()>;
}

impl Verify for VMInner {
    fn verify(&mut self, expression: &mut Expression) -> ArtResult<()> {
        let mut result = Ok(());
        for opcode in expression.opcodes.iter() {
            result = result.and(
                match opcode {
                    &DspOpcode::Unit { unit_id, .. } => {
                        self.verify_unit((expression.id, unit_id))
                    },
                    _ => Ok(())
                }
            );

            if result.is_err() {
                expression.state = ExpressionState::Free;
                break;
            }
        }

        result
    }
}
