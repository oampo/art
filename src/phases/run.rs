use std::mem;

use types::ArtResult;
use errors::InvalidByteCodeError;

use vm_inner::VMInner;
use opcode::DspOpcode;
use expression::Expression;

use instructions::dsp::unit::Unit;
use instructions::dsp::dac::Dac;
use instructions::dsp::parameter::Parameter;

use util::SwapExpression;

pub trait Run {
    fn run(&mut self, adc_block: &[f32], dac_block: &mut [f32]);
    fn run_expression(&mut self, id: u32, adc_block: &[f32],
                      dac_block: &mut[f32]) -> ArtResult<()>;
}

impl Run for VMInner {
    fn run(&mut self, adc_block: &[f32], dac_block: &mut [f32]) {
        let mut expression_ids = Vec::<u32>::with_capacity(0);
        mem::swap(&mut self.expression_ids, &mut expression_ids);
        for id in expression_ids.iter() {
            let result = self.run_expression(*id, adc_block, dac_block);
            result.unwrap_or_else(|error| error!("{:?}", error));
        }
        mem::swap(&mut self.expression_ids, &mut expression_ids);
    }

    fn run_expression(&mut self, id: u32, adc_block: &[f32],
                      dac_block: &mut[f32]) -> ArtResult<()> {
        let mut expression = Expression::new(Vec::with_capacity(0));
        self.expressions.swap(id, &mut expression);

        for opcode in expression.opcodes.iter() {
            match opcode {
                &DspOpcode::Unit { unit_id } => {
                    try!(self.tick_unit(unit_id))
                },
                &DspOpcode::Dac => {
                    try!(self.tick_dac(dac_block));
                },
                &DspOpcode::Parameter { unit_id, parameter_id } => {
                    try!(self.tick_parameter(unit_id, parameter_id));
                }
                _ => return Err(InvalidByteCodeError::new())
            }
        }

        self.expressions.swap(id, &mut expression);
        Ok(())
    }
}
