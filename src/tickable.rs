use unit_definition::UnitDefinition;

pub trait Tickable {
    fn tick(&mut self, block: &mut[f32]);
    fn get_definition(&self) -> &UnitDefinition;

    fn get_input_channels(&self) -> u32 {
        self.get_definition().input_channels
    }

    fn get_output_channels(&self) -> u32 {
        self.get_definition().output_channels
    }
}

pub type TickableBox = Box<Tickable + 'static>;
pub type TickableConstructor = fn(u32, u32) -> TickableBox;
