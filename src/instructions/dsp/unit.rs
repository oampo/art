use types::ArtResult;
use errors::UnitNotFoundError;
use vm_inner::VMInner;

pub trait Unit {
    fn init_unit(&mut self, id: u32, owner_id: u32) -> ArtResult<()>;
    fn tick_unit(&mut self, id: u32) -> ArtResult<()>;
}

impl Unit for VMInner {
    fn init_unit(&mut self, id: u32, owner_id: u32) -> ArtResult<()> {
        let unit = try!(
            self.units.get_mut(&id).ok_or(
                UnitNotFoundError::new(id)
            )
        );

        unit.owner = Some(owner_id);
        Ok(())
    }

    fn tick_unit(&mut self, id: u32) -> ArtResult<()> {
        let mut unit = try!(
            self.units.get_mut(&id).ok_or(
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
            end = self.channel_stack.position;
            try!(self.channel_stack.pop_expect(input_channels));
            start = self.channel_stack.position;
        }

        if output_channels != 0u32 {
            try!(self.channel_stack.push(output_channels));
            end = self.channel_stack.position;
        }

        // Split the stack into the unit half, and half which the unit
        // can use for whatever
        let (unit_stack, stack) = self.channel_stack.data.split_at_mut(end);
        let mut block = unit_stack.slice_from_mut(start);

        try!(
            (unit.tick)(block, &unit.layout, &mut unit.data,
                        stack, &mut self.busses)
        );

        Ok(())
    }
}
