use types::{ArtResult, ParameterMap};
use channel_stack::ChannelStack;

#[derive(Copy)]
pub struct ChannelLayout {
    pub input: u32,
    pub output: u32
}

pub type TickFunction = fn(
    unit: &mut Unit, block: &mut[f32], parameters: &mut ParameterMap,
    stack: &mut ChannelStack, busses: &mut ChannelStack
) -> ArtResult<()>;

#[derive(Copy)]
pub struct UnitDefinition {
    pub name: &'static str,
    pub min_channels: ChannelLayout,
    pub max_channels: ChannelLayout,
    // TODO: Remove me when we describe parameters properly
    pub num_parameters: u32,
    pub tick: TickFunction
}

#[derive(Copy)]
pub struct Unit {
    pub id: (u32, u32),
    pub definition: &'static UnitDefinition,
    pub layout: ChannelLayout,
    pub data: UnitData
}

#[derive(Copy)]
pub enum UnitData {
    Sine {
        position: f32,
    },
    // Stops irrefutable if-let error.  Remove when another unit is introduced.
    Unknown
}

