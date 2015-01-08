use std::default::Default;

#[derive(Copy)]
pub struct VMOptions {
    pub input_device: int,
    pub output_device: int,
    pub input_channels: u32,
    pub output_channels: u32
}

impl Default for VMOptions {
    fn default() -> VMOptions {
        VMOptions {
            input_device: -1,
            output_device: -1,
            input_channels: 1,
            output_channels: 1
        }
    }
}
