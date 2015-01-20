use types::ArtResult;
use opcode::DspOpcode;
use vm_inner::VMInner;
use expression::Expression;

pub trait AddExpression {
    fn add_expression(&mut self, id: u32, opcodes: Vec<DspOpcode>)
            -> ArtResult<()>;
}

impl AddExpression for VMInner {
    fn add_expression(&mut self, id: u32, opcodes: Vec<DspOpcode>)
            -> ArtResult<()> {
        let expression = Expression::new(opcodes);
        self.expressions.insert(id, expression);
        Ok(())
    }
}
