use std::collections::HashMap;

use types::ArtResult;
use errors::UndefinedUnitError;
use tickable::{TickableBox, TickableConstructor};
use dsp::oscillators::sine;

pub struct UnitFactory {
    unit_map: HashMap<u32, TickableConstructor>,
}

impl UnitFactory {
    pub fn new() -> UnitFactory {
        let mut factory = UnitFactory {unit_map: HashMap::new()};
        factory.register(sine::Sine::new_boxed);
        factory
    }

    pub fn register(&mut self, constructor: TickableConstructor) {
        let type_id = self.unit_map.len();
        debug!("Registering tickable: type_id = {}", type_id);
        self.unit_map.insert(type_id as u32, constructor);
    }

    pub fn create(&mut self, type_id: u32, input_channels: u32,
                  output_channels: u32) -> ArtResult<TickableBox> {
        debug!("Creating tickable: type_id = {}, \
                input_channels = {}, output_channels = {}",
               type_id, input_channels, output_channels);

        let constructor = try!(
            self.unit_map.get(&type_id).ok_or(UndefinedUnitError::new(type_id))
        );
        Ok((*constructor)(input_channels, output_channels))
    }
}


