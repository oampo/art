use std::default::Default;
use device::DeviceId;

#[derive(Copy)]
pub struct Options {
    pub input_device: DeviceId,
    pub output_device: DeviceId,

    pub input_channels: u32,
    pub output_channels: u32,

    pub block_size: u32,
    pub sample_rate: u32,

    pub num_stack_channels: u32,
    pub num_bus_channels: u32,

    pub max_opcodes: u32,
    pub max_expressions: u32,
    pub max_units: u32,
    pub max_parameters: u32,
    pub max_edges: u32
}

impl Default for Options {
    fn default() -> Options {
        Options {
            input_device: DeviceId::Default,
            output_device: DeviceId::Default,
            input_channels: 2,
            output_channels: 2,
            block_size: 64,
            sample_rate: 44100,
            num_stack_channels: 32,
            num_bus_channels: 32,
            max_opcodes: 1024,
            max_expressions: 32,
            max_units: 128,
            max_parameters: 256,
            max_edges: 32
        }
    }
}
