use types::ArtResult;
use sizes::BLOCK_SIZE;
use channel_stack::ChannelStack;

#[derive(Copy)]
pub struct DACInstruction;

impl DACInstruction {
    pub fn run(channels: &mut ChannelStack, block: &mut[f32])
            -> ArtResult<()> {
        let end = channels.position;
        try!(channels.pop_expect((block.len() / BLOCK_SIZE) as u32));
        let start = channels.position;

        let slice = channels.data.slice_mut(start, end);

        assert!(slice.len() == block.len());

        for i in range(0, slice.len()) {
            block[i] = slice[i];
        }

        Ok(())
    }
}
