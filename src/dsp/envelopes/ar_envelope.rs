use types::{ArtResult, Rate};

use unit::{Unit, UnitDefinition, ChannelLayout, UnitKind, DataSize,
           TickAdjuncts};
use parameter::{ParameterDefinition, ParameterMode};
use channel_stack::ChannelStack;
use leap::Leap;
use constants::Constants;

pub static PARAMETERS_AR: [ParameterDefinition; 3] = [
    ParameterDefinition {
        name: "gate",
        default: 1f32,
        rate: Rate::Audio,
        mode: ParameterMode::Trigger
    },
    ParameterDefinition {
        name: "attack",
        default: 1f32,
        rate: Rate::Control,
        mode: ParameterMode::Normal
    },
    ParameterDefinition {
        name: "release",
        default: 1f32,
        rate: Rate::Control,
        mode: ParameterMode::Normal
    }
];

pub static DEFINITION_AR: UnitDefinition = UnitDefinition {
    name: "ar_envelope_ar",
    kind: UnitKind::Source,
    input_rate: None,
    output_rate: Some(Rate::Audio),
    default_layout: ChannelLayout {
        input: 0,
        output: 1
    },
    parameters: &PARAMETERS_AR,
    tick: ArEnvelopeAr::tick,
    data_size: DataSize::Fixed(3)
};

#[derive(Copy)]
pub struct ArEnvelopeAr;

impl ArEnvelopeAr {
    pub fn new(id: (u32, u32), input_channels: u32, output_channels: u32,
               data: &mut Leap<f32>)
            -> Unit {
        let data_index = data.tail;
        data.push(0.0).unwrap(); // value
        data.push(0.0).unwrap(); // delta
        data.push(0.0).unwrap(); // last_gate

        Unit {
            definition: &DEFINITION_AR,
            id: id,
            layout: ChannelLayout {
                input: input_channels,
                output: output_channels
            },
            data_index: Some(data_index)
        }
    }

    fn tick(unit: &mut Unit, block: &mut[f32], parameters: &mut ChannelStack,
            adjuncts: &mut TickAdjuncts, constants: &Constants)
            -> ArtResult<()> {
        debug_assert!(unit.data_index.is_some());
        let mut data = adjuncts.data.iter_mut(unit.data_index.unwrap());
        let value = data.next().unwrap();
        let delta = data.next().unwrap();
        let last_gate = data.next().unwrap();

        let (mut gate_stack, others) = parameters.split_at_mut(
            constants.block_size
        );
        let gate_chock = gate_stack.get_mut(0, constants.block_size);
        let attack = others.data[0];
        let release = others.data[1];

        let channels = unit.layout.output as usize;

        for i in range(0, constants.block_size) {
            let gate = gate_chock[i];
            if gate > 0.0 && *last_gate <= 0.0 {
                if attack == 0.0 {
                    *delta = 1.0 - *value;
                }
                else {
                    *delta = (1.0 - *value) /
                            (attack * constants.audio_rate);
                }
            }

            *value += *delta;

            if *value >= 1.0 {
                *value = 1.0;
                if release == 0.0 {
                    *delta = -*value;
                }
                else {
                    *delta = -*value / (release * constants.audio_rate);
                }
            }

            if *value <= 0.0 {
                *value = 0.0;
                *delta = 0.0;
            }

            *last_gate = gate;

            for j in range(0, channels) {
                block[i * channels + j] = *value;
            }
        }
        Ok(())
    }
}

pub static PARAMETERS_KR: [ParameterDefinition; 3] = [
    ParameterDefinition {
        name: "gate",
        default: 1f32,
        rate: Rate::Control,
        mode: ParameterMode::Trigger
    },
    ParameterDefinition {
        name: "attack",
        default: 1f32,
        rate: Rate::Control,
        mode: ParameterMode::Normal
    },
    ParameterDefinition {
        name: "release",
        default: 1f32,
        rate: Rate::Control,
        mode: ParameterMode::Normal
    }
];

pub static DEFINITION_KR: UnitDefinition = UnitDefinition {
    name: "ar_envelope_kr",
    kind: UnitKind::Source,
    input_rate: None,
    output_rate: Some(Rate::Control),
    default_layout: ChannelLayout {
        input: 0,
        output: 1
    },
    parameters: &PARAMETERS_KR,
    tick: ArEnvelopeKr::tick,
    data_size: DataSize::Fixed(3)
};

#[derive(Copy)]
pub struct ArEnvelopeKr;

impl ArEnvelopeKr {
    pub fn new(id: (u32, u32), input_channels: u32, output_channels: u32,
               data: &mut Leap<f32>)
            -> Unit {
        let data_index = data.tail;
        data.push(0.0).unwrap(); // value
        data.push(0.0).unwrap(); // delta
        data.push(0.0).unwrap(); // last_gate

        Unit {
            definition: &DEFINITION_KR,
            id: id,
            layout: ChannelLayout {
                input: input_channels,
                output: output_channels
            },
            data_index: Some(data_index)
        }
    }

    fn tick(unit: &mut Unit, block: &mut[f32], parameters: &mut ChannelStack,
            adjuncts: &mut TickAdjuncts, constants: &Constants)
            -> ArtResult<()> {
        debug_assert!(unit.data_index.is_some());
        let mut data = adjuncts.data.iter_mut(unit.data_index.unwrap());
        let value = data.next().unwrap();
        let delta = data.next().unwrap();
        let last_gate = data.next().unwrap();

        let gate = parameters.data[0];
        let attack = parameters.data[1];
        let release = parameters.data[2];

        let channels = unit.layout.output as usize;

        if gate > 0.0 && *last_gate <= 0.0 {
            if attack == 0.0 {
                *delta = 1.0 - *value;
            }
            else {
                *delta = (1.0 - *value) /
                        (attack * constants.control_rate);
            }
        }

        *value += *delta;

        if *value >= 1.0 {
            *value = 1.0;
            if release == 0.0 {
                *delta = -*value;
            }
            else {
                *delta = -*value / (release * constants.control_rate);
            }
        }

        if *value <= 0.0 {
            *value = 0.0;
            *delta = 0.0;
        }

        *last_gate = gate;

        for i in range(0, channels) {
            block[i] = *value;
        }
        Ok(())
    }
}
