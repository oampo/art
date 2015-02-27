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

    pub max_stack_depth: u32,
    pub max_bus_depth: u32,
    pub stack_size: usize,
    pub bus_stack_size: usize,

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
            max_stack_depth: 32,
            max_bus_depth: 32,
            stack_size: 32 * 64,
            bus_stack_size: 32 * 64,
            max_opcodes: 1024,
            max_expressions: 32,
            max_units: 128,
            max_parameters: 256,
            max_edges: 32
        }
    }
}
