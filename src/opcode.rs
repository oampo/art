#[deriving(FromPrimitive)]
pub enum Opcode {
    CreateUnit,
    SetParameter,
    Expression,
    Play,
    Unit,
    Parameter,
    Sample
}
