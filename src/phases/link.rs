use std::mem;

use types::ArtResult;

use vm_inner::VMInner;
use expression::Expression;
use opcode::DspOpcode;
use util::SwapExpression;

use instructions::dsp::parameter::Parameter;

pub trait Link {
    fn link(&mut self);
    fn link_expression(&mut self, expression_id: u32) -> ArtResult<()>;
}

impl Link for VMInner {
    fn link(&mut self) {
        let mut expression_ids = Vec::<u32>::with_capacity(0);
        mem::swap(&mut self.expression_ids, &mut expression_ids);
        for &id in expression_ids.iter() {
            let result = self.link_expression(id);
            result.unwrap_or_else(|error| error!("{:?}", error));
        }
        mem::swap(&mut self.expression_ids, &mut expression_ids);
    }

    fn link_expression(&mut self, expression_id: u32) -> ArtResult<()> {
        let mut expression = Expression::new(Vec::with_capacity(0));
        self.expressions.swap(expression_id, &mut expression);

        for opcode in expression.opcodes.iter() {
            match opcode {
                &DspOpcode::Parameter { unit_id, id } => {
                    try!(
                        self.link_parameter(unit_id, id, expression_id)
                    )
                },
                _ => {}
            }
        }

        self.expressions.swap(expression_id, &mut expression);
        Ok(())
    }
}
