use types::ArtResult;

use vm_inner::VmInner;
use channel_stack::ChannelStack;

pub trait Dac {
    fn tick_dac(&mut self, block: &mut[f32], stack: &mut ChannelStack)
            -> ArtResult<()>;
}

impl Dac for VmInner {
    fn tick_dac(&mut self, block: &mut[f32], stack: &mut ChannelStack)
            -> ArtResult<()> {
        let channels = block.len() / self.constants.sizes.block_size;
        let index = try!(stack.pop(channels as u32));
        try!(stack.read(index, block));
        Ok(())
    }
}
