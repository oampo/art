use std::mem;

use types::ArtResult;

use vm_inner::VmInner;
use expression_list::ExpressionList;
use channel_stack::ChannelStack;
use opcode::DspOpcode;

use instructions::dsp::parameter::Parameter;

pub trait Link {
    fn link(&mut self, busses: &mut ChannelStack);
    fn link_expression(&mut self, expression_id: u32,
                       busses: &mut ChannelStack) -> ArtResult<()>;
}

impl Link for VmInner {
    fn link(&mut self, busses: &mut ChannelStack) {
        debug!("Starting link phase");

        // Set the expression IDs
        self.expression_ids.clear();
        for id in self.expressions.keys() {
            self.expression_ids.push(*id);
        }

        let mut expression_ids = Vec::<u32>::with_capacity(0);
        mem::swap(&mut self.expression_ids, &mut expression_ids);

        for &id in expression_ids.iter() {
            let result = self.link_expression(id, busses);
            result.unwrap_or_else(|error| error!("{}", error));
        }

        mem::swap(&mut self.expression_ids, &mut expression_ids);
    }

    fn link_expression(&mut self, from_id: u32,
                       busses: &mut ChannelStack) -> ArtResult<()> {
        let index = self.expressions.get(&from_id).unwrap().index;

        let mut expression_list = ExpressionList::new();
        mem::swap(&mut self.expression_list, &mut expression_list);

        for opcode in try!(expression_list.iter(index)) {
            match opcode {
                DspOpcode::Parameter { expression_id, unit_id,
                                       parameter_id } => {
                    try!(
                        self.link_parameter(
                            from_id, (expression_id, unit_id, parameter_id),
                            busses
                        )
                    )
                },
                _ => {}
            }
        }

        mem::swap(&mut self.expression_list, &mut expression_list);
        Ok(())
    }
}
