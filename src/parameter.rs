use types::{ArtResult, Rate};
use constants::Constants;

use channel_stack::ChannelStack;


#[derive(Copy)]
pub struct Parameter {
    pub definition: &'static ParameterDefinition,
    pub value: f32,
    last_value: f32,
    pub bus: Option<usize>
}

impl Parameter {
    pub fn new(definition: &'static ParameterDefinition) -> Parameter {
        Parameter {
            definition: definition,
            value: definition.default,
            last_value: definition.default,
            bus: None
        }
    }

    pub fn get(&mut self, stack: &mut ChannelStack, busses: &mut ChannelStack,
               constants: &Constants) -> ArtResult<usize> {
        let samples = match self.definition.rate {
            Rate::Audio => constants.block_size,
            Rate::Control => 1
        };
        let index = try!(stack.push(samples));
        let block = try!(stack.get(index, samples));

        match self.bus {
            Some(index) => {
                try!(busses.read(index, block));
                self.last_value = block[samples - 1];
            },
            None => {
                match self.definition.rate {
                    Rate::Audio => {
                        let delta = (self.value - self.last_value) *
                                    constants.block_size_inverse;
                        for i in range(0, constants.block_size) {
                            block[i] = self.last_value + i as f32 * delta;
                        }
                    },
                    Rate::Control => {
                        block[0] = self.value;
                    }
                }
            }
        }
        Ok(index)
    }

    pub fn tick(&mut self, stack: &mut ChannelStack, busses: &mut ChannelStack,
                constants: &Constants)
            -> ArtResult<()> {
        let samples = match self.definition.rate {
            Rate::Audio => constants.block_size,
            Rate::Control => 1
        };
        let bus_index = try!(busses.push(samples));
        let index = try!(stack.pop(samples));
        try!(busses.write(bus_index, try!(stack.get(index, samples))));
        self.bus = Some(bus_index);
        Ok(())
    }
}
#[derive(Copy, RustcEncodable)]
pub struct ParameterDefinition {
    pub name: &'static str,
    pub default: f32,
    pub rate: Rate
}

