use types::{UnitMap, ArtResult};
use errors::{InvalidByteCodeError};
use opcode::Opcode;
use channel_stack::ChannelStack;
use instructions::unit::UnitInstruction;

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

    pub fn execute(&mut self, units: &mut UnitMap) -> ArtResult<()> {
        for opcode in self.opcodes.iter() {
            match opcode {
                &Opcode::Unit { id } => {
                    try!(UnitInstruction::run(id, units, &mut self.channels))
                },
                _ => return Err(InvalidByteCodeError::new())
            }
        }
        Ok(())
    }
}
