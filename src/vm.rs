use env_logger;


use types::{ArtResult, ByteCodeReceiver};
use options::Options;
use device::{Device, Stream};
use vm_inner::VmInner;

pub struct Vm {
    inner: VmInner,
    options: Options
}

impl Vm {
    pub fn new(options: Options, input_channel: ByteCodeReceiver) -> Vm {
        env_logger::init().unwrap();
        Device::init().unwrap();
        Vm {
            inner: VmInner::new(&options, input_channel),
            options: options
        }
    }

    pub fn list() -> ArtResult<()> {
        try!(Device::init());
        try!(Device::list());
        try!(Device::uninit());
        Ok(())
    }

    pub fn start(&mut self) -> ArtResult<Stream> {
        let _ = self.inner.write_info_file();
        let constants = self.inner.constants;
        let stream = try!(
            Device::open(&self.options, &mut self.inner, constants)
        );
        try!(stream.start());
        Ok(stream)
    }
}

impl Drop for Vm {
    fn drop(&mut self) {
        Device::uninit().unwrap();
    }
}
