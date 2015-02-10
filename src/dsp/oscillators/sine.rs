use std::num::Float;
use std::f32::consts::PI_2;

use types::{ArtResult, Rate};

use unit::{Unit, UnitDefinition, UnitData, ChannelLayout, UnitKind,
           TickAdjuncts};
use parameter::ParameterDefinition;
use channel_stack::ChannelStack;
use constants::Constants;

use util::modulo;

pub static PARAMETERS_AR: [ParameterDefinition; 2] = [
    ParameterDefinition {
        name: "frequency",
        default: 440f32,
        rate: Rate::Audio
    },
    ParameterDefinition {
        name: "phase",
        default: 0f32,
        rate: Rate::Audio
    }
];

pub static DEFINITION_AR: UnitDefinition = UnitDefinition {
    name: "sine_ar",
    kind: UnitKind::Source,
    input_rate: Rate::Audio,
    output_rate: Rate::Audio,
    default_channels: ChannelLayout {
        input: 0,
        output: 1
    },
    parameters: &PARAMETERS_AR,
    tick: SineAr::tick
};

#[derive(Copy)]
pub struct SineAr;

impl SineAr {
    pub fn new(id: (u32, u32), input_channels: u32, output_channels: u32)
            -> Unit {
        Unit {
            definition: &DEFINITION_AR,
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
            _: &mut TickAdjuncts, constants: &Constants) -> ArtResult<()> {
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

pub static PARAMETERS_KR: [ParameterDefinition; 2] = [
    ParameterDefinition {
        name: "frequency",
        default: 440f32,
        rate: Rate::Control
    },
    ParameterDefinition {
        name: "phase",
        default: 0f32,
        rate: Rate::Control
    }
];

pub static DEFINITION_KR: UnitDefinition = UnitDefinition {
    name: "sine_kr",
    kind: UnitKind::Source,
    input_rate: Rate::Control,
    output_rate: Rate::Control,
    default_channels: ChannelLayout {
        input: 0,
        output: 1
    },
    parameters: &PARAMETERS_KR,
    tick: SineKr::tick
};

#[derive(Copy)]
pub struct SineKr;

impl SineKr {
    pub fn new(id: (u32, u32), input_channels: u32, output_channels: u32)
            -> Unit {
        Unit {
            definition: &DEFINITION_KR,
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
            _: &mut TickAdjuncts, constants: &Constants) -> ArtResult<()> {
        if let UnitData::Sine {ref mut position} = unit.data {
            let frequency = parameters.data[0];
            let phase = parameters.data[1];

            let channels = unit.layout.output as usize;

            let value = (*position + phase).sin();
            for i in range(0, channels) {
                block[i] = value;
            }
            *position += frequency * PI_2 * constants.control_rate_inverse;
            *position = modulo(*position, PI_2);
        }
        Ok(())
    }
}

