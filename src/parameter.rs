use types::ArtResult;
use sizes::{BLOCK_SIZE, BLOCK_SIZE_INVERSE};

use channel_stack::ChannelStack;

#[derive(Copy)]
pub struct Parameter {
    pub value: f32,
    last_value: f32,
    pub bus: Option<u32>
}

impl Parameter {
    pub fn new(value: f32) -> Parameter {
        Parameter {
            value: value,
            last_value: value,
            bus: None
        }
    }

    pub fn get(&mut self, stack: &mut ChannelStack, busses: &mut ChannelStack) 
            -> ArtResult<u32> {
        let index = try!(stack.push(1));
        let block = try!(stack.get(index, 1));
        match self.bus {
            Some(id) => {
                try!(busses.read(id, block));
            },
            None => {
                let delta = (self.value - self.last_value) * BLOCK_SIZE_INVERSE;
                for i in range(0, BLOCK_SIZE) {
                    block[i] = self.last_value + i as f32 * delta;
                }
            }
        }
        self.last_value = block[BLOCK_SIZE - 1];
        Ok(index)
    }
}
