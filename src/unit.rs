use types::ArtResult;
use parameter::Parameter;
use bus_manager::BusManager;

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

#[derive(Copy)]
pub struct ChannelLayout {
    pub input: u32,
    pub output: u32
}

pub type TickFunction = fn(
    block: &mut[f32], channels: &ChannelLayout, data: &mut UnitData,
    stack: &mut [f32], busses: &mut BusManager
) -> ArtResult<()>;

#[derive(Copy)]
pub struct Unit {
    pub layout: ChannelLayout,
    pub data: UnitData,
    pub tick: TickFunction,
    pub owner: Option<u32>
}

impl Unit {
    pub fn new(input_channels: u32, output_channels: u32, data: UnitData,
               tick: TickFunction) -> Unit {
        Unit {
            layout: ChannelLayout {
                input: input_channels,
                output: output_channels,
            },
            data: data,
            tick: tick,
            owner: None
        }
    }
}

#[derive(Copy)]
pub enum UnitData {
    Sine {
        position: f32,
        parameters: [Parameter; 2]
    },
    // Stops irrefutable if-let error.  Remove when another unit is introduced.
    Unknown
}

impl UnitData {
    pub fn get_parameters(&mut self) -> &mut [Parameter] {
        match *self {
            UnitData::Sine {ref mut parameters, ..} => parameters.as_mut_slice(),
            UnitData::Unknown => unimplemented!()
        }
    }
}

