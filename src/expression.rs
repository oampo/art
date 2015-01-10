use types::{UnitMap, ArtResult};
use errors::{InvalidByteCodeError};
use opcode::Opcode;
use channel_stack::ChannelStack;
use instructions::unit::UnitInstruction;
use instructions::dac::DACInstruction;

pub struct Expression {
    opcodes: Vec<Opcode>,
    channels: ChannelStack
}

impl Expression {
    pub fn new(opcodes: Vec<Opcode>) -> Expression {
        Expression {
            opcodes: opcodes,
            channels: ChannelStack::new()
        }
    }

    pub fn execute(&mut self, units: &mut UnitMap, adc_block: &[f32],
                   dac_block: &mut [f32]) -> ArtResult<()> {
        for opcode in self.opcodes.iter() {
            match opcode {
                &Opcode::Unit { id } => {
                    try!(UnitInstruction::run(id, units, &mut self.channels))
                },
                &Opcode::DAC => {
                    try!(DACInstruction::run(&mut self.channels, dac_block));
                },
                _ => return Err(InvalidByteCodeError::new())
            }
        }
        Ok(())
    }
}
