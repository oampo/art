use std::mem;

use types::ArtResult;

use vm_inner::VMInner;
use expression::{Expression, ExpressionState};
use expression_list::ExpressionList;
use opcode::DspOpcode;

use instructions::dsp::unit::Unit;

pub trait Verify {
    fn verify(&mut self, expression: &mut Expression) -> ArtResult<()>;
}

impl Verify for VMInner {
    fn verify(&mut self, expression: &mut Expression) -> ArtResult<()> {
        let index = expression.index;

        let mut expression_list = ExpressionList::new();
        mem::swap(&mut self.expression_list, &mut expression_list);

        let mut result = Ok(());
        for opcode in try!(expression_list.iter(index)) {
            result = result.and(
                match opcode {
                    DspOpcode::Unit { unit_id, .. } => {
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

        mem::swap(&mut self.expression_list, &mut expression_list);

        result
    }
}
