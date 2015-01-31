use std::mem;

use types::ArtResult;

use vm_inner::VMInner;
use expression::{Expression, ExpressionState};
use expression_list::ExpressionList;
use opcode::DspOpcode;

use instructions::dsp::unit::Unit;

pub trait Construct {
    fn construct(&mut self, expression: &mut Expression) -> ArtResult<()>;
}

impl Construct for VMInner {
    fn construct(&mut self, expression: &mut Expression) -> ArtResult<()> {
        let index = expression.index;

        let mut expression_list = ExpressionList::new();
        mem::swap(&mut self.expression_list, &mut expression_list);

        let mut result = Ok(());
        for opcode in try!(expression_list.iter(index)) {
            if let DspOpcode::Unit { unit_id, type_id, input_channels,
                                      output_channels } = opcode {
                result = result.and(
                    self.construct_unit((expression.id, unit_id),
                                        type_id, input_channels,
                                        output_channels)
                );

                if result.is_err() {
                    expression.state = ExpressionState::Free;
                    break;
                }
            }
        }

        mem::swap(&mut self.expression_list, &mut expression_list);

        result
    }
}
