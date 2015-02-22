use types::Rate;

#[derive(FromPrimitive, Copy, Debug)]
pub enum ControlOpcodeType {
    SetParameter,
    AddExpression,
    AddEdge,
}

#[derive(FromPrimitive, Copy, Debug)]
pub enum DspOpcodeType {
    Unit = 3,
    Add,
    Multiply
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
    AddEdge {
        from: u32,
        to: u32
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
    Add {
        channels: u32,
        rate: Rate
    },
    Multiply {
        channels: u32,
        rate: Rate
    }
}

#[derive(Copy, Debug)]
pub enum Opcode {
    Control(ControlOpcode),
    Dsp(DspOpcode)
}

