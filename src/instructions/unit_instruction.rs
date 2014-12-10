use types::ArtResult;
use errors::{UnitNotFoundError, InvalidStackError};
use instruction::Instruction;
use vm::UnitMap;
use sizes::BLOCK_SIZE;

pub struct UnitInstruction {
    unit_id: u32
}

impl UnitInstruction {
    pub fn new(unit_id: u32) -> UnitInstruction {
        UnitInstruction {
            unit_id: unit_id
        }
    }
}

impl Instruction for UnitInstruction {
    fn execute(&mut self, channels: &mut Vec<f32>,
               channel_stack: &mut Vec<u32>,
               channel_pointer: &mut u32, units: &mut UnitMap)
            -> ArtResult<()> {
        let mut unit = try!(
            units.get_mut(&self.unit_id).ok_or(
                UnitNotFoundError::new(self.unit_id)
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
                channel_stack.pop().ok_or(InvalidStackError::new())
            );

            // The number of channels at the top of the stack should be the
            // number of input channels of the unit
            if channels_top != input_channels {
                return Err(InvalidStackError::new());
            }

            end = (*channel_pointer) as uint * BLOCK_SIZE;
            *channel_pointer -= input_channels;
            start = (*channel_pointer) as uint * BLOCK_SIZE;
        }


        if output_channels > input_channels {
            let current = channels.len();
            end += (output_channels - input_channels) as uint * BLOCK_SIZE;
            if current < end {
                // Not enough channels allocated, so grow the vector
                channels.grow((end - current), 0f32);
            }
            channel_stack.push(output_channels);
            *channel_pointer += output_channels;
        }

        let mut slice = channels.slice_mut(start, end);
        unit.tick(slice);

        Ok(())
    }
}
