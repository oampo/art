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
        let block = stack.get_mut(index, samples);

        match self.definition.rate {
            Rate::Control => {
                self.read_control(block);
            },
            Rate::Audio => {
                self.read_audio(block, busses, constants);
            }
        }

        Ok(index)
    }

    fn read_control(&mut self, block: &mut [f32]) {
        block[0] = self.value;
        if let ParameterMode::Trigger = self.definition.mode {
            self.value = 0.0;
        }
    }

    fn read_audio(&mut self, block: &mut [f32], busses: &mut ChannelStack,
                  constants: &Constants) {
        if let Some(index) = self.bus {
            self.read_audio_bus(block, busses, index);
            return;
        }

        match self.definition.mode {
            ParameterMode::Normal => {
                self.read_audio_normal(block, constants);
            },
            ParameterMode::Trigger => {
                self.read_audio_trigger(block, constants);
            },
            ParameterMode::Interpolate => {
                self.read_audio_interpolate(block, constants);
            }
        }
    }

    fn read_audio_bus(&mut self, block: &mut [f32],
                      busses: &mut ChannelStack, index: usize) {
        busses.read(index, block);
        self.last_value = block[block.len() - 1];
    }

    fn read_audio_normal(&mut self, block: &mut [f32], constants: &Constants) {
        for i in range(0, constants.block_size) {
            block[i] = self.value;
        }
        self.last_value = self.value;
    }

    fn read_audio_trigger(&mut self, block: &mut [f32],
                          constants: &Constants) {
        let value = self.value;
        self.value = 0.0;
        self.read_audio_normal(block, constants);
        block[0] = value;
    }

    fn read_audio_interpolate(&mut self, block: &mut [f32],
                                   constants: &Constants) {
        let delta = (self.value - self.last_value) *
                    constants.block_size_inverse;
        for i in range(0, constants.block_size) {
            block[i] = self.last_value + i as f32 * delta;
        }
        self.last_value = self.value;
    }
}

#[derive(Copy, RustcEncodable)]
pub struct ParameterDefinition {
    pub name: &'static str,
    pub default: f32,
    pub rate: Rate,
    pub mode: ParameterMode
}

#[derive(Copy, RustcEncodable)]
pub enum ParameterMode {
    Normal,
    Trigger,
    Interpolate
}

