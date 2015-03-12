use types::{ArtResult, Rate};

use unit::{Unit, UnitDefinition, UnitData, ChannelLayout, UnitKind,
           TickAdjuncts};
use parameter::{ParameterDefinition};
use channel_stack::ChannelStack;
use constants::Constants;

pub static PARAMETERS: [ParameterDefinition; 0] = [
];

pub static DEFINITION: UnitDefinition = UnitDefinition {
    name: "kr_to_ar",
    kind: UnitKind::Processor,
    input_rate: Some(Rate::Control),
    output_rate: Some(Rate::Audio),
    default_layout: ChannelLayout {
        input: 1,
        output: 1
    },
    parameters: &PARAMETERS,
    tick: KrToAr::tick
};

#[derive(Copy)]
pub struct KrToAr;

impl KrToAr {
    pub fn new(id: (u32, u32), input_channels: u32, output_channels: u32)
            -> Unit {
        Unit {
            definition: &DEFINITION,
            id: id,
            layout: ChannelLayout {
                input: input_channels,
                output: output_channels
            },
            data: UnitData::None
        }
    }

    fn tick(unit: &mut Unit, block: &mut[f32], _: &mut ChannelStack,
            _: &mut TickAdjuncts, constants: &Constants)
                -> ArtResult<()> {

        let channels = unit.layout.output as usize;

        for i in range(1, constants.block_size) {
            for j in range(0, channels) {
                block[i * channels + j] = block[j];
            }
        }

        Ok(())
    }
}

