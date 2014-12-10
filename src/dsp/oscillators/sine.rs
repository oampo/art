use std::num::FloatMath;
use std::f32::consts::PI_2;

use sizes::BLOCK_SIZE;
use rates::AUDIO_RATE_INVERSE;
use unit_definition::UnitDefinition;
use tickable::{Tickable, TickableBox};
use util::Ascii4;
use util::modulo;

pub static TYPE_ID: &'static str = "sine";

pub struct Sine {
    definition: UnitDefinition,
    frequency: f32,
    phase: f32,
    position: f32
}

impl Sine {
    pub fn new(input_channels: u32, output_channels: u32) -> Sine {
        Sine {
            definition: UnitDefinition {
                type_id: TYPE_ID.to_ascii().to_u32(),
                input_channels: input_channels,
                output_channels: output_channels
            },
            frequency: 440.0f32,
            phase: 0.0f32,
            position: 0.0f32
        }
    }

    pub fn new_boxed(input_channels: u32,
                     output_channels: u32) -> TickableBox {
        box Sine::new(input_channels, output_channels)
    }
}

impl Tickable for Sine {
    fn tick(&mut self, block: &mut[f32]) {
        for i in range(0, BLOCK_SIZE) {
            let value = (self.position + self.phase).sin();
            for j in range(0, self.get_output_channels()) {
                block[i + j as uint * BLOCK_SIZE] = value;
            }
            self.position += self.frequency * PI_2 * AUDIO_RATE_INVERSE;
            self.position = modulo(self.position, PI_2);
        }
    }

    fn get_definition(&self) -> &UnitDefinition {
        return &self.definition;
    }
}
