use std::num::Float;
use std::f32::consts::PI_2;

use sizes::BLOCK_SIZE;
use rates::AUDIO_RATE_INVERSE;
use channel_layout::ChannelLayout;
use tickable::{Tickable, TickableBox};
use util::modulo;

#[derive(Copy)]
pub struct Sine {
    layout: ChannelLayout,
    frequency: f32,
    phase: f32,
    position: f32
}

impl Sine {
    pub fn new(input_channels: u32, output_channels: u32) -> Sine {
        Sine {
            layout: ChannelLayout {
                input: input_channels,
                output: output_channels
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
        let channels = self.get_output_channels() as uint;
        for i in range(0, BLOCK_SIZE) {
            let value = (self.position + self.phase).sin();
            for j in range(0, channels) {
                block[i * channels + j] = value;
            }
            self.position += self.frequency * PI_2 * AUDIO_RATE_INVERSE;
            self.position = modulo(self.position, PI_2);
        }
    }

    fn get_channel_layout(&self) -> &ChannelLayout {
        &self.layout
    }
}
