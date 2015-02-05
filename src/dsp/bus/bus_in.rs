use std::num::Float;

use types::{ArtResult, BusMap};

use unit::{Unit, UnitDefinition, UnitData, ChannelLayout};
use parameter::ParameterDefinition;
use channel_stack::ChannelStack;
use constants::Constants;

pub static BUS_IN_PARAMETERS: [ParameterDefinition; 1] = [
    ParameterDefinition {
        name: "bus_id",
        default: 0.
    }
];

pub static BUS_IN_DEFINITION: UnitDefinition = UnitDefinition {
    name: "bus_in",
    default_channels: ChannelLayout {
        input: 0,
        output: 1
    },
    parameters: &BUS_IN_PARAMETERS,
    tick: BusIn::tick
};

#[derive(Copy)]
pub struct BusIn;

impl BusIn {
    pub fn new(id: (u32, u32), input_channels: u32, output_channels: u32)
            -> Unit {
        Unit {
            definition: &BUS_IN_DEFINITION,
            id: id,
            layout: ChannelLayout {
                input: input_channels,
                output: output_channels
            },
            data: UnitData::None
        }
    }

    fn tick(_: &mut Unit, block: &mut[f32], parameters: &mut ChannelStack,
            busses: &mut ChannelStack, bus_map: &mut BusMap,
            _: &Constants) -> ArtResult<()> {
        let bus_id = try!(parameters.get(0, 1))[0].round() as u32;

        if let Some(&bus_index) = bus_map.get(&bus_id) {;
            try!(busses.read(bus_index, block));
        }
        else {
            for i in block {
                *i = 0f32;
            }
        }

        Ok(())
    }
}

