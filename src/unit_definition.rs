#[derive(Copy)]
pub enum UnitKind {
    Source,
    Processor,
    Sink
}

#[derive(Copy)]
pub struct UnitDefinition {
    pub name: &'static str,
    pub kind: UnitKind,
    pub min_input_channels: u32,
    pub max_input_channels: u32,
    pub min_output_channels: u32,
    pub max_output_channels: u32
}
