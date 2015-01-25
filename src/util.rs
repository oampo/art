use std::mem;
use std::num::Float;

use types::ExpressionMap;

use expression::Expression;

pub fn modulo<T:Float>(a: T, b: T) -> T {
    a - (a / b).floor() * b
}

pub trait SwapExpression {
    fn swap(&mut self, id: u32, expression: &mut Expression);
}

impl SwapExpression for ExpressionMap {
    fn swap(&mut self, id: u32, expression: &mut Expression) {
        if let Some(expression_b) = self.get_mut(&id) {
            mem::swap(expression, expression_b)
        }
    }
}
