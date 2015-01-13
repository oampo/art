use sizes::{BLOCK_SIZE, BLOCK_SIZE_INVERSE};
use types::ArtResult;
use util::CheckedSplitAt;
use errors::StackFullError;

#[derive(Copy)]
pub struct Parameter {
    value: f32,
    last_value: f32
}

impl Parameter {
    pub fn new(value: f32) -> Parameter {
        Parameter {
            value: value,
            last_value: value
        }
    }

    pub fn set(&mut self, value: f32) {
        self.value = value;
    }

    pub fn get<'a>(&'a self, stack: &'a mut[f32])
            -> ArtResult<(&mut [f32], &mut [f32])> {
        let (chock, stack) = try!(
            stack.checked_split_at_mut(BLOCK_SIZE).ok_or(StackFullError::new())
        );

        let delta = (self.value - self.last_value) * BLOCK_SIZE_INVERSE;
        for i in range(0, BLOCK_SIZE) {
            chock[i] = self.last_value + i as f32 * delta;
        }

        Ok((chock, stack))
    }

    pub fn enter(&mut self) {
    }

    pub fn leave(&mut self) {
        self.last_value = self.value;
    }
}
