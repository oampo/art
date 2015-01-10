use std::collections::HashMap;

use unit::Unit;
use unit_definition::UnitDefinition;
use types::{ArtResult, UnitTypeId, UnitConstructor};
use errors::{UndefinedUnitError, InvalidChannelCountError};
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
        factory.register(&sine::SINE_DEFINITION, sine::Sine::as_unit);
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

    pub fn create(&mut self, type_id: u32, input_channels: u32,
                  output_channels: u32) -> ArtResult<Box<Unit + 'static>> {
        debug!("Creating unit: type_id = {}, \
                input_channels = {}, output_channels = {}",
               type_id, input_channels, output_channels);

        let item = try!(
            self.unit_map.get(&type_id).ok_or(UndefinedUnitError::new(type_id))
        );

        if (input_channels < item.definition.min_input_channels ||
            input_channels > item.definition.max_input_channels ||
            output_channels < item.definition.min_output_channels ||
            output_channels > item.definition.max_output_channels) {
            return Err(InvalidChannelCountError::new());
        }

        Ok((item.constructor)(input_channels, output_channels))
    }
}


