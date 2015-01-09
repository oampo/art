use types::{ArtResult, UnitMap};
use errors::UnitNotFoundError;
use channel_stack::ChannelStack;

#[derive(Copy)]
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
            // If we have input channels, then the number of channels at the
            // top of the stack should be the number of input channels of the
            // unit
            end = channels.position;
            try!(channels.pop_expect(input_channels));
            start = channels.position;
        }


        if output_channels != 0u32 {
            channels.push(output_channels);
            end = channels.position;
        }

        let mut slice = channels.data.slice_mut(start, end);
        unit.tick(slice);

        Ok(())
    }
}
