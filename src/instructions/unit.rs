use types::{ArtResult, UnitMap};
use errors::UnitNotFoundError;
use channel_stack::ChannelStack;

#[derive(Copy)]
pub struct UnitInstruction;

impl UnitInstruction {
    pub fn run(channels: &mut ChannelStack, id: u32, units: &mut UnitMap)
            -> ArtResult<()> {
        let mut unit = try!(
            units.get_mut(&id).ok_or(
                UnitNotFoundError::new(id)
            )
        );

        let input_channels = unit.layout.input;
        let output_channels = unit.layout.output;

        let mut start = 0us;
        let mut end = 0us;

        if input_channels != 0u32 {
            // If we have input channels, then the number of channels at the
            // top of the stack should be the number of input channels of the
            // unit
            end = channels.position;
            try!(channels.pop_expect(input_channels));
            start = channels.position;
        }

        if output_channels != 0u32 {
            try!(channels.push(output_channels));
            end = channels.position;
        }

        let (left, right) = channels.data.split_at_mut(end);
        let mut slice = left.slice_from_mut(start);

        try!(
            (unit.tick)(slice, &unit.layout, &mut unit.data,
                        right)
        );

        Ok(())
    }
}
