use std::mem;

use types::ArtResult;
use errors::ArtError;

use vm_inner::VmInner;
use opcode::{DspOpcode, Opcode};
use expression_list::ExpressionList;
use channel_stack::ChannelStack;

use instructions::dsp::unit::Unit;
use instructions::dsp::parameter::Parameter;

pub trait Run {
    fn run(&mut self, busses: &mut ChannelStack);
    fn run_expression(&mut self, id: u32, busses: &mut ChannelStack)
            -> ArtResult<()>;
    fn run_expression_inner(&mut self, id: u32, stack_data: &mut Vec<f32>,
                            busses: &mut ChannelStack) -> ArtResult<()>;
}

impl Run for VmInner {
    fn run(&mut self, busses: &mut ChannelStack) {
        debug!("Starting run phase");

        let mut expression_ids = Vec::<u32>::with_capacity(0);
        mem::swap(&mut self.expression_ids, &mut expression_ids);

        for id in expression_ids.iter() {
            let result = self.run_expression(*id, busses);
            result.unwrap_or_else(|error| error!("{}", error));
        }

        mem::swap(&mut self.expression_ids, &mut expression_ids);
    }

    fn run_expression(&mut self, id: u32, busses: &mut ChannelStack)
            -> ArtResult<()> {
        let mut stack_data = Vec::with_capacity(0);
        mem::swap(&mut self.stack_data, &mut stack_data);

        try!(self.run_expression_inner(id, &mut stack_data, busses));

        mem::swap(&mut self.stack_data, &mut stack_data);
        Ok(())
    }

    fn run_expression_inner(&mut self, id: u32,
                            stack_data: &mut Vec<f32>,
                            busses: &mut ChannelStack)
            -> ArtResult<()> {
        let index = self.expressions.get(&id).unwrap().index;

        let mut expression_list = ExpressionList::new();
        mem::swap(&mut self.expression_list, &mut expression_list);


        let mut stack = ChannelStack::new(stack_data.as_mut_slice(),
                                          self.constants.block_size);
        for opcode in try!(expression_list.iter(index)) {
            match opcode {
                DspOpcode::Unit { unit_id, .. } => {
                    try!(self.tick_unit((id, unit_id), &mut stack,
                                        busses))
                },
                DspOpcode::Parameter { expression_id, unit_id,
                                        parameter_id } => {
                    try!(self.tick_parameter((expression_id,
                                              unit_id, parameter_id),
                                             &mut stack, busses));
                },
                _ => {
                    return Err(ArtError::UnimplementedOpcode {
                        opcode: Opcode::Dsp(opcode)
                    });
                }
            }
        }

        mem::swap(&mut self.expression_list, &mut expression_list);
        Ok(())
    }
}
