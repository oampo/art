use std::num::Float;
use std::f32::consts::PI_2;

use types::{ArtResult, Rate};

use unit::{Unit, UnitDefinition, ChannelLayout, UnitKind, DataSize,
           TickAdjuncts};
use parameter::{ParameterDefinition, ParameterMode};
use channel_stack::ChannelStack;
use leap::Leap;
use constants::Constants;

use util::modulo;

pub static PARAMETERS_AR: [ParameterDefinition; 2] = [
    ParameterDefinition {
        name: "frequency",
        default: 440f32,
        rate: Rate::Audio,
        mode: ParameterMode::Interpolate
    },
    ParameterDefinition {
        name: "phase",
        default: 0f32,
        rate: Rate::Audio,
        mode: ParameterMode::Interpolate
    }
];

pub static DEFINITION_AR: UnitDefinition = UnitDefinition {
    name: "sine_ar",
    kind: UnitKind::Source,
    input_rate: None,
    output_rate: Some(Rate::Audio),
    default_layout: ChannelLayout {
        input: 0,
        output: 1
    },
    parameters: &PARAMETERS_AR,
    tick: SineAr::tick,
    data_size: DataSize::Fixed(1)
};

#[derive(Copy)]
pub struct SineAr;

impl SineAr {
    pub fn new(id: (u32, u32), input_channels: u32, output_channels: u32,
               data: &mut Leap<f32>)
            -> Unit {
        let data_index = data.tail;
        data.push(0.0).unwrap();

        Unit {
            definition: &DEFINITION_AR,
            id: id,
            layout: ChannelLayout {
                input: input_channels,
                output: output_channels
            },
            data_index: Some(data_index)
        }
    }

    fn tick(unit: &mut Unit, block: &mut[f32], parameters: &mut ChannelStack,
            adjuncts: &mut TickAdjuncts, constants: &Constants)
            -> ArtResult<()> {
        debug_assert!(unit.data_index.is_some());
        let mut data = adjuncts.data.iter_mut(unit.data_index.unwrap());
        let position = data.next().unwrap();

        let (mut frequency_stack,
             mut phase_stack) = parameters.split_at_mut(
           constants.block_size
        );
        let frequency = frequency_stack.get_mut(0, constants.block_size);
        let phase = phase_stack.get_mut(0, constants.block_size);

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
        Ok(())
    }
}

pub static PARAMETERS_KR: [ParameterDefinition; 2] = [
    ParameterDefinition {
        name: "frequency",
        default: 440f32,
        rate: Rate::Control,
        mode: ParameterMode::Normal
    },
    ParameterDefinition {
        name: "phase",
        default: 0f32,
        rate: Rate::Control,
        mode: ParameterMode::Normal
    }
];

pub static DEFINITION_KR: UnitDefinition = UnitDefinition {
    name: "sine_kr",
    kind: UnitKind::Source,
    input_rate: None,
    output_rate: Some(Rate::Control),
    default_layout: ChannelLayout {
        input: 0,
        output: 1
    },
    parameters: &PARAMETERS_KR,
    tick: SineKr::tick,
    data_size: DataSize::Fixed(1)
};

#[derive(Copy)]
pub struct SineKr;

impl SineKr {
    pub fn new(id: (u32, u32), input_channels: u32, output_channels: u32,
               data: &mut Leap<f32>)
            -> Unit {
        let data_index = data.tail;
        data.push(0.0).unwrap();
        Unit {
            definition: &DEFINITION_KR,
            id: id,
            layout: ChannelLayout {
                input: input_channels,
                output: output_channels
            },
            data_index: Some(data_index)
        }
    }

    fn tick(unit: &mut Unit, block: &mut[f32], parameters: &mut ChannelStack,
            adjuncts: &mut TickAdjuncts, constants: &Constants)
            -> ArtResult<()> {
        debug_assert!(unit.data_index.is_some());
        let mut data = adjuncts.data.iter_mut(unit.data_index.unwrap());
        let position = data.next().unwrap();

        let frequency = parameters.data[0];
        let phase = parameters.data[1];

        let channels = unit.layout.output as usize;

        let value = (*position + phase).sin();
        for i in range(0, channels) {
            block[i] = value;
        }
        *position += frequency * PI_2 * constants.control_rate_inverse;
        *position = modulo(*position, PI_2);
        Ok(())
    }
}

