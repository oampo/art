use std::num::Float;

use types::{ArtResult, BusMap};

use unit::{Unit, UnitDefinition, UnitData, ChannelLayout};
use parameter::ParameterDefinition;
use channel_stack::ChannelStack;
use constants::Constants;

pub static BUS_OUT_PARAMETERS: [ParameterDefinition; 1] = [
    ParameterDefinition {
        name: "bus_id",
        default: 0.
    }
];

pub static BUS_OUT_DEFINITION: UnitDefinition = UnitDefinition {
    name: "bus_in",
    default_channels: ChannelLayout {
        input: 1,
        output: 0
    },
    parameters: &BUS_OUT_PARAMETERS,
    tick: BusOut::tick
};

#[derive(Copy)]
pub struct BusOut;

impl BusOut {
    pub fn new(id: (u32, u32), input_channels: u32, output_channels: u32)
            -> Unit {
        Unit {
            definition: &BUS_OUT_DEFINITION,
            id: id,
            layout: ChannelLayout {
                input: input_channels,
                output: output_channels
            },
            data: UnitData::None
        }
    }

    fn tick(unit: &mut Unit, block: &mut[f32], parameters: &mut ChannelStack,
            busses: &mut ChannelStack, bus_map: &mut BusMap,
            _: &Constants) -> ArtResult<()> {
        let bus_id = try!(parameters.get(0, 1))[0].round() as u32;

        if let Some(&bus_index) = bus_map.get(&bus_id) {
            try!(busses.add(bus_index, block));
        }
        else {
            let bus_index = try!(busses.push(unit.layout.input));
            try!(busses.write(bus_index, block));
            bus_map.insert(bus_id, bus_index);
        }

        Ok(())
    }
}

