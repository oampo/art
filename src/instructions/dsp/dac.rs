use types::ArtResult;
use sizes::BLOCK_SIZE;

use vm_inner::VMInner;
use channel_stack::ChannelStack;

pub trait Dac {
    fn tick_dac(&mut self, block: &mut[f32], stack: &mut ChannelStack)
            -> ArtResult<()>;
}

impl Dac for VMInner {
    fn tick_dac(&mut self, block: &mut[f32], stack: &mut ChannelStack)
            -> ArtResult<()> {
        let channels = block.len() / BLOCK_SIZE;
        let index = try!(stack.pop(channels as u32));
        try!(stack.read(index, block));
        Ok(())
    }
}
