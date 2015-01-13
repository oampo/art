use types::ArtResult;
use parameter::Parameter;

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
    parameter_stack: &mut [f32]
) -> ArtResult<()>;

pub struct Unit {
    pub layout: ChannelLayout,
    pub data: UnitData,
    pub tick: TickFunction
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
            tick: tick
        }
    }

    pub fn enter(&mut self) {
        for parameter in self.data.get_parameters().iter_mut() {
            parameter.enter();
        }
    }

    pub fn leave(&mut self) {
        for parameter in self.data.get_parameters().iter_mut() {
            parameter.leave();
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
    fn get_parameters(&mut self) -> &mut [Parameter] {
        match *self {
            UnitData::Sine {ref mut parameters, ..} => parameters.as_mut_slice(),
            UnitData::Unknown => unimplemented!()
        }
    }
}

