use types::ArtResult;
use std::num::Float;
use std::f32::consts::PI_2;
use std::u32;

use unit::{Unit, UnitDefinition, UnitKind, UnitData, ChannelLayout};
use sizes::BLOCK_SIZE;
use rates::AUDIO_RATE_INVERSE;
use parameter::Parameter;
use util::modulo;

pub static SINE_DEFINITION: UnitDefinition = UnitDefinition {
    name: "Sine",
    kind: UnitKind::Source,
    min_input_channels: 0,
    max_input_channels: 0,
    min_output_channels: 1,
    max_output_channels: u32::MAX
};

#[derive(Copy])
pub struct Sine;

impl Sine {
    pub fn new(input_channels: u32, output_channels: u32) -> Unit {
        Unit::new(
            input_channels,
            output_channels,
            UnitData::Sine {
                position: 0.0,
                parameters: [Parameter::new(440.0), Parameter::new(0.0)],
            },
            Sine::tick
        )
    }

    fn tick(block: &mut[f32], layout: &ChannelLayout, data: &mut UnitData,
            stack: &mut [f32]) -> ArtResult<()> {
        if let &mut UnitData::Sine {ref mut position,
                                    ref mut parameters} = data {

            let (left, right) = parameters.split_at_mut(1);
            let (frequency, stack) = try!(left[0].get(stack));
            let (phase, _) = try!(right[0].get(stack));
            let channels = layout.output as usize;

            for i in range(0, BLOCK_SIZE) {
                let value = (*position + phase[i]).sin();
                for j in range(0, channels) {
                    block[i * channels + j] = value;
                }
                *position += frequency[i] * PI_2 * AUDIO_RATE_INVERSE;
                *position = modulo(*position, PI_2);
            }
        }
        Ok(())
    }
}

