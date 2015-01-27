use std::mem;

use types::ArtResult;

use vm_inner::VMInner;
use expression::{Expression, ExpressionState};
use opcode::DspOpcode;
use util::SwapExpression;

use instructions::dsp::unit::Unit;

pub trait Verify {
    fn verify(&mut self);
    fn verify_expression(&mut self, expression_id: u32) -> ArtResult<()>;
}

impl Verify for VMInner {
    fn verify(&mut self) {
        debug!("Starting verify phase");

        // Set the expression IDs
        self.expression_ids.clear();
        for (id, expression) in self.expressions.iter() {
            if let ExpressionState::Verify = expression.state {
                self.expression_ids.push(*id);
            }
        }

        let mut expression_ids = Vec::<u32>::with_capacity(0);
        mem::swap(&mut self.expression_ids, &mut expression_ids);

        for &id in expression_ids.iter() {
            let result = self.verify_expression(id);
            result.unwrap_or_else(|error| error!("{}", error));
        }

        mem::swap(&mut self.expression_ids, &mut expression_ids);
    }

    fn verify_expression(&mut self, expression_id: u32) -> ArtResult<()> {
        let mut expression = Expression::new(Vec::with_capacity(0));
        self.expressions.swap(expression_id, &mut expression);

        let mut result = Ok(());
        for opcode in expression.opcodes.iter() {
            result = result.and(
                match opcode {
                    &DspOpcode::Unit { unit_id, .. } => {
                        self.verify_unit((expression_id, unit_id))
                    },
                    _ => Ok(())
                }
            );

            if result.is_err() {
                expression.state = ExpressionState::Free;
                break;
            }
        }

        self.expressions.swap(expression_id, &mut expression);
        result
    }
}
