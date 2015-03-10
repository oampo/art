use types::{ArtResult, UnitMap, ParameterMap};
use unit::TickAdjuncts;
use constants::Constants;
use opcode::{DspOpcode};
use unit_factory::UnitFactory;
use channel_stack::ChannelStack;
use leap::Leap;
use operators;

#[derive(Copy)]
pub enum ExpressionState {
    Run,
    Free
}

#[derive(Copy)]
pub struct Expression {
    pub id: u32,
    pub index: usize,
    pub num_opcodes: usize,
    pub incoming_edges: u32,
    pub state: ExpressionState
}

impl Expression {
    pub fn new(id: u32, index: usize, num_opcodes: usize) -> Expression {
        Expression {
            id: id,
            index: index,
            num_opcodes: num_opcodes,
            incoming_edges: 0,
            state: ExpressionState::Run
        }
    }

    pub fn construct_units(&self, store: &Leap<DspOpcode>,
                           factory: &mut UnitFactory, units: &mut UnitMap,
                           parameters: &mut ParameterMap) {
        for opcode in store.iter(self.index).take(self.num_opcodes) {
            if let &DspOpcode::Unit { unit_id, type_id, input_channels,
                                      output_channels } = opcode {
                let unit = factory.create((self.id, unit_id), type_id,
                                          input_channels, output_channels);
                unit.construct_parameters(parameters);
                debug_assert!(units.len() < units.capacity());
                units.insert((self.id, unit_id), unit);
            }
        }
    }

    pub fn tick(&self, store: &Leap<DspOpcode>, stack: &mut ChannelStack,
                units: &mut UnitMap, adjuncts: &mut TickAdjuncts,
                constants: &Constants) -> ArtResult<()> {
        for opcode in store.iter(self.index).take(self.num_opcodes) {
            match opcode {
                &DspOpcode::Unit { unit_id, .. } => {
                    debug_assert!(units.contains_key(&(self.id, unit_id)));
                    let mut unit = units.get_mut(&(self.id, unit_id)).unwrap();
                    try!(
                        unit.tick(stack, adjuncts,
                                  constants)
                    );
                },
                &DspOpcode::Add { channels, rate } => {
                    try!(operators::add(stack, channels, rate, constants));
                },
                &DspOpcode::Multiply { channels, rate } => {
                    try!(operators::multiply(stack, channels, rate, constants));
                }
            }
        }
        Ok(())
    }
}

