use std::default::Default;

use types::{ArtResult, ByteCodeReceiver};
use device::{Device, DeviceId};
use vm_inner::VMInner;

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

pub struct VM<'a> {
    inner: VMInner,
    device: Device<'a>
}

impl<'a> VM<'a> {
    pub fn new(options: VMOptions, input_channel: ByteCodeReceiver)
            -> VM<'a> {
        VM {
            inner: VMInner::new(input_channel),
            device:  Device::new(options.input_device, options.output_device,
                                 options.input_channels,
                                 options.output_channels)
        }
    }

    pub fn start(&'a mut self) -> ArtResult<()> {
        if !self.device.is_open() {
            try!(self.device.open(&mut self.inner));
        }

        try!(self.device.start());
        Ok(())
    }
}

