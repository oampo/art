use std::mem;

use types::ArtResult;

use vm_inner::VMInner;
use expression::Expression;
use opcode::DspOpcode;
use util::SwapExpression;

use instructions::dsp::unit::Unit;

pub trait Init {
    fn init(&mut self);
    fn init_expression(&mut self, expression_id: u32) -> ArtResult<()>;
}

impl Init for VMInner {
    fn init(&mut self) {
        debug!("Starting init phase");

        // Set the expression IDs
        self.expression_ids.clear();
        for id in self.expressions.keys() {
            self.expression_ids.push(*id);
        }

        let mut expression_ids = Vec::<u32>::with_capacity(0);
        mem::swap(&mut self.expression_ids, &mut expression_ids);

        for &id in expression_ids.iter() {
            let result = self.init_expression(id);
            result.unwrap_or_else(|error| error!("{:?}", error));
        }

        mem::swap(&mut self.expression_ids, &mut expression_ids);
    }

    fn init_expression(&mut self, expression_id: u32) -> ArtResult<()> {
        let mut expression = Expression::new(Vec::with_capacity(0));
        self.expressions.swap(expression_id, &mut expression);

        for opcode in expression.opcodes.iter() {
            match opcode {
                &DspOpcode::Unit { unit_id } => {
                    try!(
                        self.init_unit(unit_id, expression_id)
                    )
                },
                _ => {}
            }
        }

        self.expressions.swap(expression_id, &mut expression);
        Ok(())
    }
}
