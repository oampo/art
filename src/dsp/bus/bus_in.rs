use std::num::Float;

use types::{ArtResult, Rate};

use unit::{Unit, UnitDefinition, UnitData, UnitKind, ChannelLayout,
           TickAdjuncts};
use parameter::ParameterDefinition;
use channel_stack::ChannelStack;
use constants::Constants;

pub static PARAMETERS: [ParameterDefinition; 1] = [
    ParameterDefinition {
        name: "bus_id",
        default: 0f32,
        rate: Rate::Control
    }
];

#[derive(Copy)]
pub struct BusIn;


impl BusIn {
    fn tick(_: &mut Unit, block: &mut[f32], parameters: &mut ChannelStack,
            adjuncts: &mut TickAdjuncts, _: &Constants) -> ArtResult<()> {
        let bus_id = parameters.data[0].round() as u32;

        if let Some(&bus_index) = adjuncts.bus_map.get(&bus_id) {
            try!(adjuncts.busses.read(bus_index, block));
        }
        else {
            for i in block {
                *i = 0f32;
            }
        }

        Ok(())
    }
}

pub static DEFINITION_AR: UnitDefinition = UnitDefinition {
    name: "bus_in_ar",
    kind: UnitKind::Source,
    input_rate: Rate::Audio,
    output_rate: Rate::Audio,
    default_channels: ChannelLayout {
        input: 0,
        output: 1
    },
    parameters: &PARAMETERS,
    tick: BusIn::tick
};

#[derive(Copy)]
pub struct BusInAr;

impl BusInAr {
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
}

pub static DEFINITION_KR: UnitDefinition = UnitDefinition {
    name: "bus_in_kr",
    kind: UnitKind::Source,
    input_rate: Rate::Control,
    output_rate: Rate::Control,
    default_channels: ChannelLayout {
        input: 0,
        output: 1
    },
    parameters: &PARAMETERS,
    tick: BusIn::tick
};

#[derive(Copy)]
pub struct BusInKr;

impl BusInKr {
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
}

