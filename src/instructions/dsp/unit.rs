use std::cmp;

use types::ArtResult;
use errors::ArtError;

use vm_inner::VMInner;
use channel_stack::ChannelStack;

pub trait Unit {
    fn init_unit(&mut self, unit_id: u32, owner_id: u32) -> ArtResult<()>;
    fn tick_unit(&mut self, unit_id: u32, stack: &mut ChannelStack,
                 busses: &mut ChannelStack) -> ArtResult<()>;
}

impl Unit for VMInner {
    fn init_unit(&mut self, unit_id: u32, owner_id: u32) -> ArtResult<()> {
        let unit = try!(
            self.units.get_mut(&unit_id).ok_or(
                ArtError::UnitNotFound {
                    unit_id: unit_id
                }
            )
        );

        unit.owner = Some(owner_id);
        Ok(())
    }

    fn tick_unit(&mut self, unit_id: u32, stack: &mut ChannelStack,
                 busses: &mut ChannelStack)
            -> ArtResult<()> {
        let mut unit = try!(
            self.units.get_mut(&unit_id).ok_or(
                ArtError::UnitNotFound {
                    unit_id: unit_id
                }
            )
        );

        let input_channels = unit.layout.input;
        let output_channels = unit.layout.output;
        let channels = cmp::max(input_channels, output_channels);

        let index = try!(stack.pop(input_channels));
        try!(stack.push(output_channels));


        // Split the stack into the unit half, and half which the unit
        // can use for whatever
        let (mut unit_stack, mut stack) = stack.split(
            index + channels
        );
        let mut block = try!(unit_stack.get(index, channels));

        try!(
            (unit.tick)(block, &unit.layout, &mut unit.data,
                        &mut stack, busses)
        );

        Ok(())
    }
}
