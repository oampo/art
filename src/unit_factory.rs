use rustc_serialize::{Encoder, Encodable};

use unit::{Unit, UnitDefinition};
use types::{UnitConstructor};

use dsp::oscillators::sine::{self, SineAr, SineKr};
use dsp::bus::bus_in::{self, BusInAr, BusInKr};
use dsp::bus::bus_out::{self, BusOutAr, BusOutKr};
use dsp::parameter::parameter::{self, ParameterAr, ParameterKr};
use dsp::parameter::parameter_writer::{self, ParameterWriterAr,
                                       ParameterWriterKr};

#[derive(Copy)]
pub struct UnitFactoryItem {
    definition: &'static UnitDefinition,
    constructor: UnitConstructor
}

impl Encodable for UnitFactoryItem {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        self.definition.encode(encoder)
    }
}

pub struct UnitFactory {
    pub units: Vec<UnitFactoryItem>
}

impl UnitFactory {
    pub fn new() -> UnitFactory {
        let mut factory = UnitFactory {units: Vec::new()};
        factory.register(&sine::DEFINITION_AR, SineAr::new);
        factory.register(&sine::DEFINITION_KR, SineKr::new);
        factory.register(&bus_in::DEFINITION_AR, BusInAr::new);
        factory.register(&bus_in::DEFINITION_KR, BusInKr::new);
        factory.register(&bus_out::DEFINITION_AR, BusOutAr::new);
        factory.register(&bus_out::DEFINITION_KR, BusOutKr::new);
        factory.register(&parameter::DEFINITION_AR, ParameterAr::new);
        factory.register(&parameter::DEFINITION_KR, ParameterKr::new);
        factory.register(&parameter_writer::DEFINITION_AR,
                         ParameterWriterAr::new);
        factory.register(&parameter_writer::DEFINITION_KR,
                         ParameterWriterKr::new);
        factory
    }

    pub fn register(&mut self, definition: &'static UnitDefinition,
                    constructor: UnitConstructor) {
        self.units.push(
            UnitFactoryItem {
                constructor: constructor,
                definition: definition
            }
        );
    }

    pub fn is_registered(&self, type_id: u32) -> bool {
        return (type_id as usize) < self.units.len();
    }

    pub fn create(&mut self, id: (u32, u32), type_id: u32,
                  input_channels: u32, output_channels: u32) -> Unit {
        debug_assert!(self.is_registered(type_id));
        (self.units[type_id as usize].constructor)(id, input_channels,
                                                    output_channels)
    }

    pub fn get_definition(&self, type_id: u32) -> &UnitDefinition {
        debug_assert!(self.is_registered(type_id));
        self.units[type_id as usize].definition
    }
}


