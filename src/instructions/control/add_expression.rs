use types::ArtResult;

use vm_inner::VmInner;
use expression::Expression;

use phases::verify::Verify;
use phases::construct::Construct;

pub trait AddExpression {
    fn add_expression(&mut self, id: u32, index: usize)
            -> ArtResult<()>;
}

impl AddExpression for VmInner {
    fn add_expression(&mut self, id: u32, index: usize)
            -> ArtResult<()> {
        debug!("Adding expression: id={:?}, index={:?}", id, index);
        let mut expression = Expression::new(id, index);

        try!(self.verify(&mut expression));

        let result = self.construct(&mut expression);
        // Insert even if construction fails so we free up any units and
        // parameters which were sucessfully constructed
        self.expressions.insert(id, expression);
        result
    }
}
