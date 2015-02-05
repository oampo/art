use std::num::Float;
use std::f32::consts::PI_2;

use types::{ArtResult, BusMap};

use unit::{Unit, UnitDefinition, UnitData, ChannelLayout};
use parameter::ParameterDefinition;
use channel_stack::ChannelStack;
use constants::Constants;

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
    default_channels: ChannelLayout {
        input: 0,
        output: 1
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

    fn tick(unit: &mut Unit, block: &mut[f32], parameters: &mut ChannelStack,
            _: &mut ChannelStack, _: &mut BusMap, constants: &Constants)
                -> ArtResult<()> {
        if let UnitData::Sine {ref mut position} = unit.data {
            let (mut frequency_stack, mut phase_stack) = parameters.split(1);
            let frequency = try!(frequency_stack.get(0, 1));
            let phase = try!(phase_stack.get(0, 1));

            let channels = unit.layout.output as usize;

            for i in range(0, constants.block_size) {
                let value = (*position + phase[i]).sin();
                for j in range(0, channels) {
                    block[i * channels + j] = value;
                }
                *position += frequency[i] * PI_2 *
                             constants.audio_rate_inverse;
                *position = modulo(*position, PI_2);
            }
        }
        Ok(())
    }
}

