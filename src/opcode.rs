#[derive(FromPrimitive, Copy, Show)]
pub enum OpcodeType {
    CreateUnit,
    SetParameter,
    Expression,
    Play,
    Unit,
    Parameter,
    Sample,
    DAC,
    ADC,
    Unknown
}

#[derive(Show)]
pub enum Opcode {
    CreateUnit {
        id: u32,
        type_id: u32,
        input_channels: u32,
        output_channels: u32
    },
    SetParameter {
        unit_id: u32,
        parameter_id: u32,
        value: f32
    },
    Expression {
        id: u32,
        opcodes: Vec<Opcode>
    },
    Play {
        id: u32
    },
    Unit {
        id: u32
    },
    Parameter {
        unit_id: u32,
        parameter_id: u32
    },
    Sample {
        value: f32
    },
    DAC,
    ADC,
    Unknown
}

