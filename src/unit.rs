use channel_layout::ChannelLayout;
use parameter::Parameter;

pub trait Unit {
    fn tick(&mut self, block: &mut[f32]);

    fn get_channel_layout(&self) -> &ChannelLayout;

    fn get_input_channels(&self) -> u32 {
        self.get_channel_layout().input
    }

    fn get_output_channels(&self) -> u32 {
        self.get_channel_layout().output
    }

    fn get_parameters(&mut self) -> &mut [Parameter];

    fn enter(&mut self) {
        for parameter in self.get_parameters().iter_mut() {
            parameter.enter();
        }
    }

    fn leave(&mut self) {
        for parameter in self.get_parameters().iter_mut() {
            parameter.leave();
        }
    }
}

