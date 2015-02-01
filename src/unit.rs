use types::ArtResult;
use constants::Constants;

use channel_stack::ChannelStack;
use parameter::ParameterDefinition;

#[derive(Copy)]
pub struct Unit {
    pub id: (u32, u32),
    pub definition: &'static UnitDefinition,
    pub layout: ChannelLayout,
    pub data: UnitData
}

#[derive(Copy)]
pub struct ChannelLayout {
    pub input: u32,
    pub output: u32
}

pub type TickFunction = fn(
    unit: &mut Unit, block: &mut[f32], parameters: &mut ChannelStack,
    constants: &Constants
) -> ArtResult<()>;

#[derive(Copy)]
pub struct UnitDefinition {
    pub name: &'static str,
    pub min_channels: ChannelLayout,
    pub max_channels: ChannelLayout,
    pub parameters: &'static [ParameterDefinition],
    pub tick: TickFunction
}

#[derive(Copy)]
pub enum UnitData {
    Sine {
        position: f32,
    },
    // Stops irrefutable if-let error.  Remove when another unit is introduced.
    Unknown
}


