use std::mem;
use std::num::Float;

use types::ExpressionMap;

use expression::Expression;

pub fn modulo<T:Float>(a: T, b: T) -> T {
    a - (a / b).floor() * b
}

pub trait CheckedSplitAt {
    type Item;
    fn checked_split_at(&mut self, mid: usize)
        -> Option<(&[<Self as SliceExt>::Item],
                   &[<Self as SliceExt>::Item])>;
    fn checked_split_at_mut(&mut self, mid: usize)
            -> Option<(&mut [<Self as SliceExt>::Item],
                       &mut [<Self as SliceExt>::Item])>;
}

impl <T> CheckedSplitAt for [T] {
    type Item = T;

    fn checked_split_at(&mut self, mid: usize)
        -> Option<(&[T], &[T])> {
        if mid > self.len() {
            return None;
        }
        Some(self.split_at(mid))
    }


    fn checked_split_at_mut(&mut self, mid: usize)
            -> Option<(&mut [T], &mut [T])> {
        if mid > self.len() {
            return None;
        }
        Some(self.split_at_mut(mid))
    }
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
