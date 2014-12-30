#[deriving(Copy)]
pub struct UnitDefinition {
    pub type_id: &'static str,
    pub input_channels: u32,
    pub output_channels: u32
}
