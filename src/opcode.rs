#[deriving(FromPrimitive, Copy)]
pub enum OpcodeType {
    CreateUnit,
    SetParameter,
    Expression,
    Play,
    Unit,
    Parameter,
    Sample
}

pub enum Opcode {
    CreateUnit {
        id: u32,
        type_id: u32,
        input_channels: u32,
        output_channels: u32
    },
    SetParameter,
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
    Parameter,
    Sample
}
