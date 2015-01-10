use types::{ArtResult, ByteCodeReceiver};
use vm_options::VMOptions;
use device::Device;
use vm_inner::VMInner;

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

