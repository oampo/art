use types::{UnitMap, ArtResult};
use errors::{InvalidByteCodeError};
use opcode::Opcode;
use channel_stack::ChannelStack;
use instructions::unit::UnitInstruction;
use instructions::dac::DACInstruction;
use instructions::parameter::ParameterInstruction;

pub struct Expression {
    opcodes: Vec<Opcode>,
    pub incoming_edges: u32
}

impl Expression {
    pub fn new(opcodes: Vec<Opcode>) -> Expression {
        Expression {
            opcodes: opcodes,
            incoming_edges: 0
        }
    }

    pub fn execute(&self, channels: &mut ChannelStack,
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
                &Opcode::Parameter { unit_id, id } => {
                    try!(ParameterInstruction::run(unit_id, id, units));
                }
                _ => return Err(InvalidByteCodeError::new())
            }
        }
        Ok(())
    }
}
