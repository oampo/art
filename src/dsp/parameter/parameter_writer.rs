use std::num::Float;

use types::{ArtResult, Rate};
use errors::ArtError;

use unit::{Unit, UnitDefinition, ChannelLayout, UnitKind, DataSize,
           TickAdjuncts};
use parameter::{ParameterDefinition, ParameterMode};
use channel_stack::ChannelStack;
use leap::Leap;
use constants::Constants;

pub static PARAMETERS: [ParameterDefinition; 3] = [
    ParameterDefinition {
        name: "eid",
        default: 0f32,
        rate: Rate::Control,
        mode: ParameterMode::Normal
    },
    ParameterDefinition {
        name: "uid",
        default: 0f32,
        rate: Rate::Control,
        mode: ParameterMode::Normal
    },
    ParameterDefinition {
        name: "pid",
        default: 0f32,
        rate: Rate::Control,
        mode: ParameterMode::Normal
    }
];

pub static DEFINITION_AR: UnitDefinition = UnitDefinition {
    name: "parameter_writer_ar",
    kind: UnitKind::Sink,
    input_rate: Some(Rate::Audio),
    output_rate: None,
    default_layout: ChannelLayout {
        input: 1,
        output: 0
    },
    parameters: &PARAMETERS,
    tick: ParameterWriterAr::tick,
    data_size: DataSize::None
};

#[derive(Copy)]
pub struct ParameterWriterAr;

impl ParameterWriterAr {
    pub fn new(id: (u32, u32), input_channels: u32, output_channels: u32,
               _: &mut Leap<f32>)
            -> Unit {
        Unit {
            definition: &DEFINITION_AR,
            id: id,
            layout: ChannelLayout {
                input: input_channels,
                output: output_channels
            },
            data_index: None
        }
    }

    fn tick(_: &mut Unit, block: &mut[f32], parameters: &mut ChannelStack,
            adjuncts: &mut TickAdjuncts, constants: &Constants)
                -> ArtResult<()> {
        let eid = parameters.data[0].round() as u32;
        let uid = parameters.data[1].round() as u32;
        let pid = parameters.data[2].round() as u32;

        let parameter = try!(
            adjuncts.parameters.get_mut(&(eid, uid, pid)).ok_or(
                ArtError::ParameterNotFound {
                    expression_id: eid,
                    unit_id: uid,
                    parameter_id: pid as u32
                }
            )
        );

        match parameter.definition.rate {
            Rate::Audio => {
                let bus_index = try!(
                    adjuncts.busses.push(constants.block_size)
                );
                adjuncts.busses.write(bus_index, block);
                parameter.bus = Some(bus_index);
            },
            Rate::Control => {
                parameter.value = block[0];
            }
        }
        Ok(())
    }
}

pub static DEFINITION_KR: UnitDefinition = UnitDefinition {
    name: "parameter_writer_kr",
    kind: UnitKind::Sink,
    input_rate: Some(Rate::Control),
    output_rate: None,
    default_layout: ChannelLayout {
        input: 1,
        output: 0
    },
    parameters: &PARAMETERS,
    tick: ParameterWriterKr::tick,
    data_size: DataSize::None
};

#[derive(Copy)]
pub struct ParameterWriterKr;

impl ParameterWriterKr {
    pub fn new(id: (u32, u32), input_channels: u32, output_channels: u32,
               _: &mut Leap<f32>)
            -> Unit {
        Unit {
            definition: &DEFINITION_KR,
            id: id,
            layout: ChannelLayout {
                input: input_channels,
                output: output_channels
            },
            data_index: None
        }
    }

    fn tick(_: &mut Unit, block: &mut[f32], parameters: &mut ChannelStack,
            adjuncts: &mut TickAdjuncts, _: &Constants)
                -> ArtResult<()> {
        let eid = parameters.data[0].round() as u32;
        let uid = parameters.data[1].round() as u32;
        let pid = parameters.data[2].round() as u32;

        let parameter = try!(
            adjuncts.parameters.get_mut(&(eid, uid, pid)).ok_or(
                ArtError::ParameterNotFound {
                    expression_id: eid,
                    unit_id: uid,
                    parameter_id: pid as u32
                }
            )
        );
        parameter.value = block[0];
        Ok(())
    }
}

