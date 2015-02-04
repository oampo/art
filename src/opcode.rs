#[derive(FromPrimitive, Copy, Debug)]
pub enum ControlOpcodeType {
    SetParameter,
    AddExpression,
    Play
}

#[derive(FromPrimitive, Copy, Debug)]
pub enum DspOpcodeType {
    Unit = 3,
    Parameter,
    Sample,
    Dac,
    Adc
}

#[derive(Copy, Debug)]
pub enum ControlOpcode {
    SetParameter {
        expression_id: u32,
        unit_id: u32,
        parameter_id: u32,
        value: f32
    },
    AddExpression {
        expression_id: u32,
        num_opcodes: u32
    },
    Play {
        expression_id: u32
    }
}

#[derive(Copy, Debug)]
pub enum DspOpcode {
    Unit {
        unit_id: u32,
        type_id: u32,
        input_channels: u32,
        output_channels: u32
    },
    Parameter {
        expression_id: u32,
        unit_id: u32,
        parameter_id: u32
    },
    Sample {
        value: f32
    },
    Dac,
    Adc
}

#[derive(Copy, Debug)]
pub enum Opcode {
    Control(ControlOpcode),
    Dsp(DspOpcode)
}

