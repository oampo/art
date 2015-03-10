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

    pub fn read(&mut self, stack: &mut ChannelStack, busses: &mut ChannelStack,
               constants: &Constants) -> ArtResult<usize> {
        let samples = match self.definition.rate {
            Rate::Audio => constants.block_size,
            Rate::Control => 1
        };
        let index = try!(stack.push(samples));
        let block = stack.get(index, samples);

        match self.definition.rate {
            Rate::Control => {
                block[0] = self.value;
            },
            Rate::Audio => {
                if let Some(index) = self.bus {
                    busses.read(index, block);
                    self.last_value = block[samples - 1];
                }
                else {
                    let delta = (self.value - self.last_value) *
                                constants.block_size_inverse;
                    for i in range(0, constants.block_size) {
                        block[i] = self.last_value + i as f32 * delta;
                    }
                    self.last_value = self.value;
                }
            }
        }

        Ok(index)
    }
}

#[derive(Copy, RustcEncodable)]
pub struct ParameterDefinition {
    pub name: &'static str,
    pub default: f32,
    pub rate: Rate
}

