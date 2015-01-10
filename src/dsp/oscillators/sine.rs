use std::num::Float;
use std::f32::consts::PI_2;

use unit::Unit;
use sizes::BLOCK_SIZE;
use rates::AUDIO_RATE_INVERSE;
use channel_layout::ChannelLayout;
use parameter::Parameter;
use util::modulo;

#[derive(Copy)]
pub struct Sine {
    layout: ChannelLayout,
    position: f32,
    parameters: [Parameter; 2]
}

impl Sine {
    pub fn new(input_channels: u32, output_channels: u32) -> Sine {
        Sine {
            layout: ChannelLayout {
                input: input_channels,
                output: output_channels
            },
            parameters: [Parameter::new(440.0), Parameter::new(0.0)],
            position: 0.0,
        }
    }

    pub fn as_unit(input_channels: u32, output_channels: u32)
            -> Box<Unit + 'static> {
        Box::new(Sine::new(input_channels, output_channels))
    }
}

impl Unit for Sine {
    fn tick(&mut self, block: &mut[f32]) {
        let channels = self.get_output_channels() as usize;

        let (l, r) = self.parameters.split_at_mut(1);
        let frequency = &mut l[0];
        let phase = &mut r[0];

        for i in range(0, BLOCK_SIZE) {
            let value = (self.position + phase.get(i)).sin();
            for j in range(0, channels) {
                block[i * channels + j] = value;
            }
            self.position += frequency.get(i) * PI_2 * AUDIO_RATE_INVERSE;
            self.position = modulo(self.position, PI_2);
        }
    }

    fn get_channel_layout(&self) -> &ChannelLayout {
        &self.layout
    }

    fn get_parameters(&mut self) -> &mut [Parameter] {
        &mut self.parameters
    }
}

