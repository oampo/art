use types::{ArtResult, Rate};

use unit::{Unit, UnitDefinition, UnitData, ChannelLayout, UnitKind,
           TickAdjuncts};
use parameter::{ParameterDefinition, ParameterMode};
use channel_stack::ChannelStack;
use constants::Constants;

pub static PARAMETERS_AR: [ParameterDefinition; 1] = [
    ParameterDefinition {
        name: "value",
        default: 0f32,
        rate: Rate::Audio,
        mode: ParameterMode::Interpolate
    }
];

pub static DEFINITION_AR: UnitDefinition = UnitDefinition {
    name: "parameter_ar",
    kind: UnitKind::Source,
    input_rate: None,
    output_rate: Some(Rate::Audio),
    default_layout: ChannelLayout {
        input: 0,
        output: 1
    },
    parameters: &PARAMETERS_AR,
    tick: ParameterAr::tick
};

#[derive(Copy)]
pub struct ParameterAr;

impl ParameterAr {
    pub fn new(id: (u32, u32), input_channels: u32, output_channels: u32)
            -> Unit {
        Unit {
            definition: &DEFINITION_AR,
            id: id,
            layout: ChannelLayout {
                input: input_channels,
                output: output_channels
            },
            data: UnitData::None
        }
    }

    fn tick(unit: &mut Unit, block: &mut[f32], parameters: &mut ChannelStack,
            _: &mut TickAdjuncts, constants: &Constants)
                -> ArtResult<()> {

        let channels = unit.layout.output as usize;

        for i in range(0, constants.block_size) {
            for j in range(0, channels) {
                block[i * channels + j] = parameters.data[i];
            }
        }

        Ok(())
    }
}

pub static PARAMETERS_KR: [ParameterDefinition; 1] = [
    ParameterDefinition {
        name: "value",
        default: 0f32,
        rate: Rate::Control,
        mode: ParameterMode::Normal
    }
];

pub static DEFINITION_KR: UnitDefinition = UnitDefinition {
    name: "parameter_kr",
    kind: UnitKind::Source,
    input_rate: None,
    output_rate: Some(Rate::Control),
    default_layout: ChannelLayout {
        input: 0,
        output: 1
    },
    parameters: &PARAMETERS_KR,
    tick: ParameterKr::tick
};

#[derive(Copy)]
pub struct ParameterKr;

impl ParameterKr {
    pub fn new(id: (u32, u32), input_channels: u32, output_channels: u32)
            -> Unit {
        Unit {
            definition: &DEFINITION_KR,
            id: id,
            layout: ChannelLayout {
                input: input_channels,
                output: output_channels
            },
            data: UnitData::None
        }
    }

    fn tick(unit: &mut Unit, block: &mut[f32], parameters: &mut ChannelStack,
            _: &mut TickAdjuncts, _: &Constants)
                -> ArtResult<()> {
        let channels = unit.layout.output as usize;
        for i in range(0, channels) {
            block[i] = parameters.data[0];
        }
        Ok(())
    }
}

