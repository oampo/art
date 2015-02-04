use rustc_serialize::{Encodable, Encoder};
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

#[derive(Copy, RustcEncodable)]
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
    pub default_channels: ChannelLayout,
    pub parameters: &'static [ParameterDefinition],
    pub tick: TickFunction
}

impl Encodable for UnitDefinition {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_struct("UnitDefinition", 3, |encoder| {
            try!(
                encoder.emit_struct_field("name", 0, |encoder|
                    self.name.encode(encoder)
                )
            );
            try!(
                encoder.emit_struct_field("default_channels", 1, |encoder|
                    self.default_channels.encode(encoder)
                )
            );
            try!(
                encoder.emit_struct_field("parameters", 2, |encoder|
                    self.parameters.encode(encoder)
                )
            );
            Ok(())
        })
    }
}

#[derive(Copy)]
pub enum UnitData {
    Sine {
        position: f32,
    },
    // Stops irrefutable if-let error.  Remove when another unit is introduced.
    Unknown
}


