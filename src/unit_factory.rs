use rustc_serialize::{Encoder, Encodable};

use unit::{Unit, UnitDefinition};
use types::{ArtResult, UnitConstructor};
use errors::ArtError;

use dsp::oscillators::sine::{self, SineAr};
/*use dsp::bus::bus_in::{self, BusIn};
use dsp::bus::bus_out::{self, BusOut};*/

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
/*        factory.register(&bus_in::DEFINITION, BusIn::new);
        factory.register(&bus_out::DEFINITION, BusOut::new);*/
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

    pub fn create(&mut self, id: (u32, u32), type_id: u32,
                  input_channels: u32, output_channels: u32)
            -> ArtResult<Unit> {
        if type_id as usize >= self.units.len() {
            return Err(
                ArtError::UndefinedUnit {
                    type_id: type_id
                }
            );
        }

        Ok((self.units[type_id as usize].constructor)(id, input_channels,
                                                      output_channels))
    }
}


