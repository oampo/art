use types::ArtResult;
use constants::Constants;

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

    pub fn get(&mut self, stack: &mut ChannelStack, busses: &mut ChannelStack,
               constants: &Constants) -> ArtResult<u32> {
        let index = try!(stack.push(1));
        let block = try!(stack.get(index, 1));
        match self.bus {
            Some(id) => {
                try!(busses.read(id, block));
            },
            None => {
                let delta = (self.value - self.last_value) *
                            constants.block_size_inverse;
                for i in range(0, constants.block_size) {
                    block[i] = self.last_value + i as f32 * delta;
                }
            }
        }
        self.last_value = block[constants.block_size - 1];
        Ok(index)
    }

    pub fn tick(&mut self, stack: &mut ChannelStack, busses: &mut ChannelStack)
            -> ArtResult<()> {
        let bus_id = try!(busses.push(1));
        let index = try!(stack.pop(1));
        try!(busses.write(bus_id, try!(stack.get(index, 1))));
        self.bus = Some(bus_id);
        Ok(())
    }
}
#[derive(Copy, RustcEncodable)]
pub struct ParameterDefinition {
    pub name: &'static str,
    pub default: f32
}

