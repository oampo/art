use std::mem;

use types::ArtResult;
use errors::ArtError;
use sizes::BLOCK_SIZE;

use vm_inner::VMInner;
use opcode::{DspOpcode, Opcode};
use expression::Expression;
use channel_stack::ChannelStack;

use instructions::dsp::unit::Unit;
use instructions::dsp::dac::Dac;
use instructions::dsp::parameter::Parameter;

use util::SwapExpression;

pub trait Run {
    fn run(&mut self, busses: &mut ChannelStack,
           adc_block: &[f32], dac_block: &mut [f32]);
    fn run_expression(&mut self, id: u32, busses: &mut ChannelStack,
                          adc_block: &[f32], dac_block: &mut[f32])
            -> ArtResult<()>;
    fn run_expression_inner(&mut self, id: u32,
                            stack_data: &mut Vec<f32>,
                            busses: &mut ChannelStack,
                            adc_block: &[f32], dac_block: &mut[f32])
            -> ArtResult<()>;
}

impl Run for VMInner {
    fn run(&mut self, busses: &mut ChannelStack,
           adc_block: &[f32], dac_block: &mut [f32]) {
        debug!("Starting run phase");

        let mut expression_ids = Vec::<u32>::with_capacity(0);
        mem::swap(&mut self.expression_ids, &mut expression_ids);

        for id in expression_ids.iter() {
            let result = self.run_expression(*id, busses,
                                             adc_block, dac_block);
            result.unwrap_or_else(|error| error!("{}", error));
        }

        mem::swap(&mut self.expression_ids, &mut expression_ids);
    }

    fn run_expression(&mut self, id: u32, busses: &mut ChannelStack,
                          adc_block: &[f32], dac_block: &mut[f32])
            -> ArtResult<()> {
        let mut stack_data = Vec::with_capacity(0);
        mem::swap(&mut self.stack_data, &mut stack_data);

        try!(self.run_expression_inner(id, &mut stack_data, busses, adc_block,
                                       dac_block));

        mem::swap(&mut self.stack_data, &mut stack_data);
        Ok(())
    }

    fn run_expression_inner(&mut self, id: u32,
                            stack_data: &mut Vec<f32>,
                            busses: &mut ChannelStack,
                            adc_block: &[f32], dac_block: &mut[f32])
            -> ArtResult<()> {
        let mut expression = Expression::new(0, Vec::with_capacity(0));
        self.expressions.swap(id, &mut expression);

        let mut stack = ChannelStack::new(stack_data.as_mut_slice(),
                                          BLOCK_SIZE);
        for opcode in expression.opcodes.iter() {
            match opcode {
                &DspOpcode::Unit { unit_id, .. } => {
                    try!(self.tick_unit((id, unit_id), &mut stack,
                                        busses))
                },
                &DspOpcode::Dac => {
                    try!(self.tick_dac(dac_block, &mut stack));
                },
                &DspOpcode::Parameter { expression_id, unit_id,
                                        parameter_id } => {
                    try!(self.tick_parameter((expression_id,
                                              unit_id, parameter_id),
                                             &mut stack, busses));
                },
                &DspOpcode::Unknown => {
                    return Err(ArtError::InvalidByteCode { error: None });
                },
                _ => {
                    return Err(ArtError::UnimplementedOpcode {
                        opcode: Opcode::Dsp(*opcode)
                    });
                }
            }
        }

        self.expressions.swap(id, &mut expression);
        Ok(())
    }
}
