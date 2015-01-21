#[derive(FromPrimitive, Copy, Show)]
pub enum OpcodeType {
    CreateUnit,
    SetParameter,
    AddExpression,
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
        unit_id: u32,
        type_id: u32,
        input_channels: u32,
        output_channels: u32
    },
    SetParameter {
        unit_id: u32,
        parameter_id: u32,
        value: f32
    },
    AddExpression {
        expression_id: u32,
        opcodes: Vec<DspOpcode>
    },
    Play {
        expression_id: u32
    },
    Unknown
}

#[derive(Copy, Show)]
pub enum DspOpcode {
    Unit {
        unit_id: u32
    },
    Parameter {
        unit_id: u32,
        parameter_id: u32
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

