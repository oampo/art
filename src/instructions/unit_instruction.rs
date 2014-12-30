use types::ArtResult;
use errors::{UnitNotFoundError, InvalidStackError};
use vm::UnitMap;
use sizes::BLOCK_SIZE;
use channel_stack::ChannelStack;

#[deriving(Copy)]
pub struct UnitInstruction;

impl UnitInstruction {
    pub fn run(id: u32, units: &mut UnitMap, channels: &mut ChannelStack)
            -> ArtResult<()> {
        let mut unit = try!(
            units.get_mut(&id).ok_or(
                UnitNotFoundError::new(id)
            )
        );

        let input_channels = unit.get_input_channels();
        let output_channels = unit.get_output_channels();

        let mut start = 0u;
        let mut end = 0u;

        if input_channels != 0u32 {
            // If we have input channels, then there should be something already
            // on the channels stack
            let channels_top = try!(
                channels.stack.pop().ok_or(InvalidStackError::new())
            );

            // The number of channels at the top of the stack should be the
            // number of input channels of the unit
            if channels_top != input_channels {
                return Err(InvalidStackError::new());
            }

            end = channels.position as uint * BLOCK_SIZE;
            channels.position -= input_channels;
            start = channels.position as uint * BLOCK_SIZE;
        }


        if output_channels > input_channels {
            let current = channels.data.len();
            end += (output_channels - input_channels) as uint * BLOCK_SIZE;
            if current < end {
                // Not enough channels allocated, so grow the vector
                channels.data.grow((end - current), 0f32);
            }
            channels.stack.push(output_channels);
            channels.position += output_channels;
        }

        let mut slice = channels.data.slice_mut(start, end);
        unit.tick(slice);

        Ok(())
    }
}
