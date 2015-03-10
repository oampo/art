use std::cmp;

use rustc_serialize::{Encodable, Encoder};
use types::{ArtResult, Rate, BusMap, ParameterMap};
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
                              Parameter::new(parameter));
        }
    }

    pub fn tick(&mut self, stack: &mut ChannelStack,
                adjuncts: &mut TickAdjuncts, constants: &Constants)
            -> ArtResult<()> {
        let input_channels = self.layout.input as usize;
        let output_channels = self.layout.output as usize;

        let input_samples = match self.definition.input_rate {
            Rate::Audio => input_channels * constants.block_size,
            Rate::Control => input_channels
        };

        let output_samples = match self.definition.output_rate {
            Rate::Audio => output_channels * constants.block_size,
            Rate::Control => output_channels
        };

        let samples = cmp::max(input_samples, output_samples);

        let index = try!(stack.pop(input_samples));
        try!(stack.push(output_samples));

        // Split the stack into the unit half, and half for the parameters
        let (mut unit_stack, mut parameter_stack) = stack.split(
            index + samples
        );

        let mut block = try!(unit_stack.get(index, samples));
        try!(self.tick_parameters(&mut parameter_stack, adjuncts,
                                  constants));
        try!(
            (self.definition.tick)(self, block, &mut parameter_stack,
                                   adjuncts, constants)
        );
        Ok(())
    }

    fn tick_parameters(&self, stack: &mut ChannelStack,
                       adjuncts: &mut TickAdjuncts,
                       constants: &Constants) -> ArtResult<()> {
        let (eid, uid) = self.id;
        for (pid, parameter) in self.definition.parameters.iter().enumerate() {
            let samples = match parameter.rate {
                Rate::Audio => constants.block_size,
                Rate::Control => 1
            };

            let index = try!(stack.push(samples));
            let (_, mut channel) = stack.split(index);

            debug_assert!(
                adjuncts.parameters.contains_key(&(eid, uid, pid as u32))
            );

            let parameter = adjuncts.parameters.get_mut(
                &(eid, uid, pid as u32)
            ).unwrap();
            try!(parameter.read(&mut channel, adjuncts.busses, constants));
        }
        Ok(())
    }
}


#[derive(Copy, RustcEncodable)]
pub enum UnitKind {
    Source,
    Processor,
    Sink
}


#[derive(Copy, RustcEncodable)]
pub struct ChannelLayout {
    pub input: u32,
    pub output: u32
}

pub struct TickAdjuncts<'a> {
    pub busses: &'a mut ChannelStack<'a>,
    pub bus_map: &'a mut BusMap,
    pub parameters: &'a mut ParameterMap
}

pub type TickFunction = fn(
    unit: &mut Unit, block: &mut[f32], parameters: &mut ChannelStack,
    adjuncts: &mut TickAdjuncts, constants: &Constants
) -> ArtResult<()>;

#[derive(Copy)]
pub struct UnitDefinition {
    pub name: &'static str,
    pub kind: UnitKind,
    pub input_rate: Rate,
    pub output_rate: Rate,
    pub default_layout: ChannelLayout,
    pub parameters: &'static [ParameterDefinition],
    pub tick: TickFunction
}

impl Encodable for UnitDefinition {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_struct("UnitDefinition", 6, |encoder| {
            try!(
                encoder.emit_struct_field("name", 0, |encoder|
                    self.name.encode(encoder)
                )
            );
            try!(
                encoder.emit_struct_field("kind", 1, |encoder|
                    self.kind.encode(encoder)
                )
            );
            try!(
                encoder.emit_struct_field("input_rate", 2, |encoder|
                    self.input_rate.encode(encoder)
                )
            );
            try!(
                encoder.emit_struct_field("output_rate", 3, |encoder|
                    self.output_rate.encode(encoder)
                )
            );
            try!(
                encoder.emit_struct_field("default_layout", 4, |encoder|
                    self.default_layout.encode(encoder)
                )
            );
            try!(
                encoder.emit_struct_field("parameters", 5, |encoder|
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


