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


        // Split the stack into the unit half, and half for the parameters
        let (mut unit_stack, mut parameter_stack) = stack.split(
            index + channels
        );

        let mut block = try!(unit_stack.get(index, channels));

        let (eid, uid) = unit.id;
        for pid in range(0, unit.definition.parameters.len()) {
            let (_, mut channel) = parameter_stack.split(pid as u32);

            let parameter = try!(
                self.parameters.get_mut(&(eid, uid, pid as u32)).ok_or(
                    ArtError::ParameterNotFound {
                        expression_id: eid,
                        unit_id: uid,
                        parameter_id: pid as u32
                    }
                )
            );
            try!(parameter.get(&mut channel, busses, &self.constants));
        }

        try!(
            (unit.definition.tick)(unit, block, &mut parameter_stack,
                                   busses, &mut self.bus_map, &self.constants)
        );

        Ok(())
    }
}
