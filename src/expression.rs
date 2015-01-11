use types::{UnitMap, ArtResult};
use errors::{InvalidByteCodeError};
use opcode::Opcode;
use channel_stack::ChannelStack;
use instructions::unit::UnitInstruction;
use instructions::dac::DACInstruction;

pub struct Expression {
    opcodes: Vec<Opcode>
}

impl Expression {
    pub fn new(opcodes: Vec<Opcode>) -> Expression {
        Expression {
            opcodes: opcodes
        }
    }

    pub fn execute(&mut self, channels: &mut ChannelStack,
                   units: &mut UnitMap, adc_block: &[f32],
                   dac_block: &mut [f32]) -> ArtResult<()> {
        for opcode in self.opcodes.iter() {
            match opcode {
                &Opcode::Unit { id } => {
                    try!(UnitInstruction::run(channels, id, units))
                },
                &Opcode::DAC => {
                    try!(DACInstruction::run(channels, dac_block));
                },
                _ => return Err(InvalidByteCodeError::new())
            }
        }
        Ok(())
    }
}
