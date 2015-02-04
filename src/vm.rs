use types::{ArtResult, ByteCodeReceiver};
use options::Options;
use device::Device;
use vm_inner::VmInner;

pub struct Vm<'a> {
    inner: VmInner,
    device: Device<'a>
}

impl<'a> Vm<'a> {
    pub fn new(options: &Options, input_channel: ByteCodeReceiver)
            -> Vm<'a> {
        Vm {
            inner: VmInner::new(options, input_channel),
            device:  Device::new(options.input_device, options.output_device,
                                 options.input_channels,
                                 options.output_channels)
        }
    }

    pub fn start(&'a mut self) -> ArtResult<()> {
        let _ = self.inner.write_info_file();
        if !self.device.is_open() {
            let constants = self.inner.constants;
            try!(self.device.open(&mut self.inner, constants));
        }

        try!(self.device.start());
        Ok(())
    }
}

