use std::collections::HashMap;

use unit::{Unit, UnitDefinition};
use types::{ArtResult, UnitTypeId, UnitConstructor};
use errors::ArtError;
use dsp::oscillators::sine;

struct UnitFactoryItem {
    definition: &'static UnitDefinition,
    constructor: UnitConstructor
}

pub struct UnitFactory {
    unit_map: HashMap<UnitTypeId, UnitFactoryItem>,
}

impl UnitFactory {
    pub fn new() -> UnitFactory {
        let mut factory = UnitFactory {unit_map: HashMap::new()};
        factory.register(&sine::SINE_DEFINITION, sine::Sine::new);
        factory
    }

    pub fn register(&mut self, definition: &'static UnitDefinition,
                    constructor: UnitConstructor) {
        let type_id = self.unit_map.len();
        debug!("Registering unit: name = {}, type_id = {}", definition.name,
               type_id);
        self.unit_map.insert(type_id as u32,
            UnitFactoryItem {
                constructor: constructor,
                definition: definition
            }
        );
    }

    pub fn create(&mut self, id: (u32, u32), type_id: u32,
                  input_channels: u32, output_channels: u32)
            -> ArtResult<Unit> {
        let item = try!(
            self.unit_map.get(&type_id).ok_or(
                ArtError::UndefinedUnit {
                    type_id: type_id
                }
            )
        );

        if input_channels < item.definition.min_channels.input ||
           input_channels > item.definition.max_channels.input ||
           output_channels < item.definition.min_channels.output ||
           output_channels > item.definition.max_channels.output {
            return Err(ArtError::InvalidChannelCount);
        }

        Ok((item.constructor)(id, input_channels,
                              output_channels))
    }
}


