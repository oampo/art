use types::{UnitMap, ArtResult};
use errors::{InvalidByteCodeError};
use opcode::DspOpcode;
use channel_stack::ChannelStack;
use instructions::dsp::unit::UnitInstruction;
use instructions::dsp::dac::DACInstruction;
use instructions::dsp::parameter::ParameterInstruction;
use graph::{Graph, Node};

pub struct Expression {
    id: u32,
    opcodes: Vec<DspOpcode>,
    incoming_edges: u32
}

impl Expression {
    pub fn new(id: u32, opcodes: Vec<DspOpcode>) -> Expression {
        Expression {
            id: id,
            opcodes: opcodes,
            incoming_edges: 0
        }
    }

    pub fn link(&self, units: &UnitMap, graph: &mut Graph) -> ArtResult<()> {
        for opcode in self.opcodes.iter() {
            match opcode {
                &DspOpcode::Parameter { unit_id, .. } => {
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
                &DspOpcode::Unit { id } => {
                    try!(UnitInstruction::run(channels, id, units))
                },
                &DspOpcode::Dac => {
                    try!(DACInstruction::run(channels, dac_block));
                },
                &DspOpcode::Parameter { unit_id, id } => {
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
