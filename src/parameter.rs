use sizes::BLOCK_SIZE_INVERSE;

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

    pub fn get(&self, i: usize) -> f32 {
        self.last_value + (self.value - self.last_value) * i as f32 * BLOCK_SIZE_INVERSE
    }

    pub fn enter(&mut self) {
    }

    pub fn leave(&mut self) {
        self.last_value = self.value;
    }
}
