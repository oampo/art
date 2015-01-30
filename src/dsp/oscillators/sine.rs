use std::num::Float;
use std::f32::consts::PI_2;
use std::u32;

use types::{ArtResult, ParameterMap};
use sizes::BLOCK_SIZE;
use rates::AUDIO_RATE_INVERSE;

use unit::{Unit, UnitDefinition, UnitData, ChannelLayout};
use parameter::ParameterDefinition;
use channel_stack::ChannelStack;

use util::modulo;

pub static SINE_PARAMETERS: [ParameterDefinition; 2] = [
    ParameterDefinition {
        name: "frequency",
        default: 440f32
    },
    ParameterDefinition {
        name: "phase",
        default: 0f32
    }
];

pub static SINE_DEFINITION: UnitDefinition = UnitDefinition {
    name: "sine",
    min_channels: ChannelLayout {
        input: 0,
        output: 1
    },
    max_channels: ChannelLayout {
        input: 0,
        output: u32::MAX
    },
    parameters: &SINE_PARAMETERS,
    tick: Sine::tick
};

#[derive(Copy)]
pub struct Sine;

impl Sine {
    pub fn new(id: (u32, u32), input_channels: u32, output_channels: u32)
            -> Unit {
        Unit {
            definition: &SINE_DEFINITION,
            id: id,
            layout: ChannelLayout {
                input: input_channels,
                output: output_channels
            },
            data: UnitData::Sine {
                position: 0.0
            }
        }
    }

    fn tick(unit: &mut Unit, block: &mut[f32], parameters: &mut ParameterMap,
            stack: &mut ChannelStack, busses: &mut ChannelStack)
            -> ArtResult<()> {
        if let UnitData::Sine {ref mut position} = unit.data {
            let (eid, uid) = unit.id;

            let (mut frequency_stack, mut phase_stack) = stack.split(1);
            let frequency_index = try!(
                parameters.get_mut(&(eid, uid, 0)).unwrap()
                          .get(&mut frequency_stack, busses)
            );
            let phase_index = try!(
                parameters.get_mut(&(eid, uid, 1)).unwrap()
                          .get(&mut phase_stack, busses)
            );

            let frequency = try!(frequency_stack.get(frequency_index, 1));
            let phase = try!(phase_stack.get(phase_index, 1));

            let channels = unit.layout.output as usize;

            for i in range(0, BLOCK_SIZE) {
                let value = (*position + phase[i]).sin();
                for j in range(0, channels) {
                    block[i * channels + j] = value;
                }
                *position += frequency[i] * PI_2 * AUDIO_RATE_INVERSE;
                *position = modulo(*position, PI_2);
            }
        }
        Ok(())
    }
}

