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
pub struct BusOut;

impl BusOut {
    fn tick(unit: &mut Unit, block: &mut[f32], parameters: &mut ChannelStack,
            adjuncts: &mut TickAdjuncts, constants: &Constants)
            -> ArtResult<()> {
        let bus_id = parameters.data[0].round() as u32;

        if let Some(&bus_index) = adjuncts.bus_map.get(&bus_id) {
            adjuncts.busses.add(bus_index, block);
        }
        else {
            let channels = unit.layout.input as usize;
            let samples = match unit.definition.input_rate {
                Rate::Audio => channels * constants.block_size,
                Rate::Control => channels
            };
            let bus_index = try!(adjuncts.busses.push(samples));
            adjuncts.busses.write(bus_index, block);
            adjuncts.bus_map.insert(bus_id, bus_index);
        }

        Ok(())
    }
}

pub static DEFINITION_AR: UnitDefinition = UnitDefinition {
    name: "bus_out_ar",
    kind: UnitKind::Sink,
    input_rate: Rate::Audio,
    output_rate: Rate::Audio,
    default_layout: ChannelLayout {
        input: 1,
        output: 0
    },
    parameters: &PARAMETERS,
    tick: BusOut::tick
};

#[derive(Copy)]
pub struct BusOutAr;

impl BusOutAr {
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
    name: "bus_out_kr",
    kind: UnitKind::Sink,
    input_rate: Rate::Control,
    output_rate: Rate::Control,
    default_layout: ChannelLayout {
        input: 1,
        output: 0
    },
    parameters: &PARAMETERS,
    tick: BusOut::tick
};

#[derive(Copy)]
pub struct BusOutKr;

impl BusOutKr {
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
