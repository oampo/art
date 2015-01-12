use sizes::{BLOCK_SIZE, BLOCK_SIZE_INVERSE};
use types::ArtResult;

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

    pub fn get<'a>(&'a mut self, stack: &'a mut[f32])
            -> ArtResult<(&mut [f32], &mut [f32])> {
        self.last_value = self.value;
        let (chock, stack) = stack.split_at_mut(BLOCK_SIZE);
        let delta = (self.value - self.last_value) * BLOCK_SIZE_INVERSE;
        for i in range(0, BLOCK_SIZE) {
            chock[i] = self.last_value + i as f32 * delta;
        }
        self.last_value = self.value;
        Ok((chock, stack))
    }
}
