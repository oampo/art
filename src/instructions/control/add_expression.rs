use types::ArtResult;

use opcode::DspOpcode;
use vm_inner::VMInner;
use expression::Expression;

use phases::verify::Verify;
use phases::construct::Construct;

pub trait AddExpression {
    fn add_expression(&mut self, id: u32, opcodes: Vec<DspOpcode>)
            -> ArtResult<()>;
}

impl AddExpression for VMInner {
    fn add_expression(&mut self, id: u32, opcodes: Vec<DspOpcode>)
            -> ArtResult<()> {
        debug!("Adding expression: id={:?}, opcodes={:?}", id, opcodes);
        let mut expression = Expression::new(id, opcodes);

        try!(self.verify(&mut expression));

        let result = self.construct(&mut expression);
        // Insert even if construction fails so we free up any units and
        // parameters which were sucessfully constructed
        self.expressions.insert(id, expression);
        result
    }
}
