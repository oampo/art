use channel_layout::ChannelLayout;

pub trait Tickable {
    fn tick(&mut self, block: &mut[f32]);
    fn get_channel_layout(&self) -> &ChannelLayout;
    fn get_input_channels(&self) -> u32 {
        self.get_channel_layout().input
    }
    fn get_output_channels(&self) -> u32 {
        self.get_channel_layout().output
    }
}

pub type TickableBox = Box<Tickable + 'static>;
pub type TickableConstructor = fn(u32, u32) -> TickableBox;
