use types::ArtResult;
use errors::ArtError;

use vm_inner::VmInner;
use channel_stack::ChannelStack;

pub trait Parameter {
    fn tick_parameter(&mut self, id: (u32, u32, u32),
                      stack: &mut ChannelStack, busses: &mut ChannelStack)
            -> ArtResult<()>;
}

impl Parameter for VmInner {
    fn tick_parameter(&mut self, id: (u32, u32, u32),
                      stack: &mut ChannelStack, busses: &mut ChannelStack)
            -> ArtResult<()> {
        let (eid, uid, pid) = id;
        let parameter = try!(
            self.parameters.get_mut(&id).ok_or(
                ArtError::ParameterNotFound {
                    expression_id: eid,
                    unit_id: uid,
                    parameter_id: pid
                }
            )
        );

        let bus_id = try!(busses.push(1));
        let index = try!(stack.pop(1));
        try!(busses.write(bus_id, try!(stack.get(index, 1))));
        parameter.bus = Some(bus_id);
        Ok(())
    }
}
