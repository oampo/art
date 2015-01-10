use std::default::Default;

use device_id::DeviceId;

#[derive(Copy)]
pub struct VMOptions {
    pub input_device: DeviceId,
    pub output_device: DeviceId,
    pub input_channels: u32,
    pub output_channels: u32
}

impl Default for VMOptions {
    fn default() -> VMOptions {
        VMOptions {
            input_device: DeviceId::Default,
            output_device: DeviceId::Default,
            input_channels: 2,
            output_channels: 2
        }
    }
}
