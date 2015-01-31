use std::cmp;

use types::ArtResult;
use errors::ArtError;

use vm_inner::VmInner;
use channel_stack::ChannelStack;
use parameter::Parameter;

pub trait Unit {
    fn verify_unit(&mut self, id: (u32, u32)) -> ArtResult<()>;
    fn construct_unit(&mut self, id: (u32, u32), type_id: u32,
                      input_channels: u32, output_channels: u32)
            -> ArtResult<()>;
    fn tick_unit(&mut self, id: (u32, u32), stack: &mut ChannelStack,
                 busses: &mut ChannelStack) -> ArtResult<()>;
}

impl Unit for VmInner {
    fn verify_unit(&mut self, id: (u32, u32)) -> ArtResult<()> {
        Ok(())
    }

    fn construct_unit(&mut self, id: (u32, u32), type_id: u32,
                      input_channels: u32, output_channels: u32)
            -> ArtResult<()> {
        let unit = try!(
            self.unit_factory.create(id, type_id, input_channels,
                                     output_channels)
        );

        let (eid, uid) = id;
        for (pid, parameter) in
                unit.definition.parameters.iter().enumerate() {
            self.parameters.insert((eid, uid, pid as u32),
                                   Parameter::new(parameter.default));
        }

        self.units.insert(id, unit);
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
