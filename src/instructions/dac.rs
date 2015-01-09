use types::ArtResult;
use sizes::BLOCK_SIZE;
use channel_stack::ChannelStack;

#[derive(Copy)]
pub struct DACInstruction;

impl DACInstruction {
    pub fn run(channels: &mut ChannelStack, output_block: &mut[f32])
            -> ArtResult<()> {
        let end = channels.position;
        try!(channels.pop_expect((output_block.len() / BLOCK_SIZE) as u32));
        let start = channels.position;

        let slice = channels.data.slice_mut(start, end);

        assert!(slice.len() == output_block.len());

        for i in range(0, slice.len()) {
            output_block[i] = slice[i];
        }

        Ok(())
    }
}
