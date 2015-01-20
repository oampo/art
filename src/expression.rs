use types::{UnitMap, ArtResult};
use errors::{InvalidByteCodeError};
use opcode::Opcode;
use channel_stack::ChannelStack;
use instructions::unit::UnitInstruction;
use instructions::dac::DACInstruction;
use instructions::parameter::ParameterInstruction;
use graph::{Graph, Node};

pub struct Expression {
    id: u32,
    opcodes: Vec<Opcode>,
    incoming_edges: u32
}

impl Expression {
    pub fn new(id: u32, opcodes: Vec<Opcode>) -> Expression {
        Expression {
            id: id,
            opcodes: opcodes,
            incoming_edges: 0
        }
    }

    pub fn link(&self, units: &UnitMap, graph: &mut Graph) -> ArtResult<()> {
        for opcode in self.opcodes.iter() {
            match opcode {
                &Opcode::Parameter { unit_id, .. } => {
                    try!(
                        ParameterInstruction::link(unit_id, self.id, units,
                                                   graph)
                    )
                },
                _ => {}
            }
        }
        Ok(())
    }

    pub fn run(&self, channels: &mut ChannelStack,
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

impl Node for Expression {
    fn get_edge_count(&self) -> u32 {
        self.incoming_edges
    }

    fn reset_edge_count(&mut self) {
        self.incoming_edges = 0;
    }

    fn increment_edge_count(&mut self) {
        self.incoming_edges += 1;
    }

    fn decrement_edge_count(&mut self) {
        self.incoming_edges -= 1;
    }
}
