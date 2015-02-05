use std::cmp;

use rustc_serialize::{Encodable, Encoder};
use types::{ArtResult, BusMap, ParameterMap};
use errors::ArtError;
use constants::Constants;

use channel_stack::ChannelStack;
use parameter::{Parameter, ParameterDefinition};

#[derive(Copy)]
pub struct Unit {
    pub id: (u32, u32),
    pub definition: &'static UnitDefinition,
    pub layout: ChannelLayout,
    pub data: UnitData
}

impl Unit {
    pub fn construct_parameters(&self, parameters: &mut ParameterMap) {
        let (eid, uid) = self.id;
        for (pid, parameter) in
                self.definition.parameters.iter().enumerate() {
            parameters.insert((eid, uid, pid as u32),
                              Parameter::new(parameter.default));
        }
    }

    pub fn tick(&mut self, stack: &mut ChannelStack, busses: &mut ChannelStack,
                parameters: &mut ParameterMap, bus_map: &mut BusMap,
                constants: &Constants) -> ArtResult<()> {
        let input_channels = self.layout.input;
        let output_channels = self.layout.output;
        let channels = cmp::max(input_channels, output_channels);

        let index = try!(stack.pop(input_channels));
        try!(stack.push(output_channels));

        // Split the stack into the unit half, and half for the parameters
        let (mut unit_stack, mut parameter_stack) = stack.split(
            index + channels
        );

        let mut block = try!(unit_stack.get(index, channels));
        try!(self.tick_parameters(&mut parameter_stack, busses, parameters,
                                  constants));

        try!(
            (self.definition.tick)(self, block, &mut parameter_stack,
                                   busses, bus_map, constants)
        );

        Ok(())
    }

    fn tick_parameters(&self, stack: &mut ChannelStack,
                       busses: &mut ChannelStack,
                       parameters: &mut ParameterMap,
                       constants: &Constants) -> ArtResult<()> {
        let (eid, uid) = self.id;
        for pid in range(0, self.definition.parameters.len()) {
            let (_, mut channel) = stack.split(pid as u32);

            let parameter = try!(
                parameters.get_mut(&(eid, uid, pid as u32)).ok_or(
                    ArtError::ParameterNotFound {
                        expression_id: eid,
                        unit_id: uid,
                        parameter_id: pid as u32
                    }
                )
            );
            try!(parameter.get(&mut channel, busses, constants));
        }
        Ok(())
    }
}

#[derive(Copy, RustcEncodable)]
pub struct ChannelLayout {
    pub input: u32,
    pub output: u32
}

pub type TickFunction = fn(
    unit: &mut Unit, block: &mut[f32], parameters: &mut ChannelStack,
    busses: &mut ChannelStack, bus_map: &mut BusMap, constants: &Constants
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
    None
}


