#[derive(FromPrimitive, Copy, Show)]
pub enum OpcodeType {
    CreateUnit,
    SetParameter,
    Expression,
    Play,
    Unit,
    Parameter,
    Sample,
    Dac,
    Adc,
    Unknown
}

#[derive(Show)]
pub enum ControlOpcode {
    CreateUnit {
        id: u32,
        type_id: u32,
        input_channels: u32,
        output_channels: u32
    },
    SetParameter {
        unit_id: u32,
        id: u32,
        value: f32
    },
    Expression {
        id: u32,
        opcodes: Vec<DspOpcode>
    },
    Play {
        id: u32
    },
    Unknown
}

#[derive(Show)]
pub enum DspOpcode {
    Unit {
        id: u32
    },
    Parameter {
        unit_id: u32,
        id: u32
    },
    Sample {
        value: f32
    },
    Dac,
    Adc,
    Unknown
}

#[derive(Show)]
pub enum Opcode {
    Control(ControlOpcode),
    Dsp(DspOpcode)
}

