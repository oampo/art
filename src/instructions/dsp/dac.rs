use types::ArtResult;
use sizes::BLOCK_SIZE;
use vm_inner::VMInner;

pub trait Dac {
    fn tick_dac(&mut self, block: &mut[f32]) -> ArtResult<()>;
}

impl Dac for VMInner {
    fn tick_dac(&mut self, block: &mut[f32]) -> ArtResult<()> {
        let end = self.channel_stack.position;
        try!(self.channel_stack.pop_expect((block.len() / BLOCK_SIZE) as u32));
        let start = self.channel_stack.position;

        let slice = &self.channel_stack.data[start..end];

        assert!(slice.len() == block.len());

        for i in range(0, slice.len()) {
            block[i] = slice[i];
        }

        Ok(())
    }
}
