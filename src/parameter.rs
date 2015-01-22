use sizes::{BLOCK_SIZE, BLOCK_SIZE_INVERSE};

use bus_manager::BusManager;

#[derive(Copy)]
pub struct Parameter {
    pub value: f32,
    last_value: f32,
    pub bus: Option<usize>
}

impl Parameter {
    pub fn new(value: f32) -> Parameter {
        Parameter {
            value: value,
            last_value: value,
            bus: None
        }
    }

    pub fn get(&mut self, values: &mut[f32], busses: &mut BusManager) {
        match self.bus {
            Some(id) => busses.get(id, values),
            None => {
                let delta = (self.value - self.last_value) * BLOCK_SIZE_INVERSE;
                for i in range(0, BLOCK_SIZE) {
                    values[i] = self.last_value + i as f32 * delta;
                }
            }
        }
        self.last_value = values[BLOCK_SIZE - 1];
    }
}
