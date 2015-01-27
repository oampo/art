use std::cmp;

use types::ArtResult;
use errors::ArtError;

use vm_inner::VMInner;
use channel_stack::ChannelStack;

pub trait Unit {
    fn verify_unit(&mut self, id: (u32, u32)) -> ArtResult<()>;
    fn tick_unit(&mut self, id: (u32, u32), stack: &mut ChannelStack,
                 busses: &mut ChannelStack) -> ArtResult<()>;
}

impl Unit for VMInner {
    fn verify_unit(&mut self, id: (u32, u32)) -> ArtResult<()> {
        Ok(())
    }

    fn tick_unit(&mut self, id: (u32, u32), stack: &mut ChannelStack,
                 busses: &mut ChannelStack)
            -> ArtResult<()> {
        let (eid, uid) = id;
        let mut unit = try!(
            self.units.get_mut(&id).ok_or(
                ArtError::UnitNotFound {
                    expression_id: eid,
                    unit_id: uid
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
            (unit.definition.tick)(unit, block, &mut self.parameters,
                                   &mut stack, busses)
        );

        Ok(())
    }
}
