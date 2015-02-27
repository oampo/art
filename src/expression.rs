use types::{ArtResult, UnitMap, ParameterMap, StackRecord};
use unit::TickAdjuncts;
use errors::ArtError;
use constants::Constants;
use opcode::{DspOpcode};
use unit_factory::UnitFactory;
use channel_stack::ChannelStack;
use leap::Leap;
use validator::ExpressionValidator;
use operators;

#[derive(Copy)]
pub enum ExpressionState {
    Verify,
    Construct,
    Link,
    Run,
    Free
}

#[derive(Copy)]
pub struct Expression {
    pub id: u32,
    pub index: usize,
    pub incoming_edges: u32,
    pub state: ExpressionState
}

impl Expression {
    pub fn new(id: u32, index: usize) -> Expression {
        Expression {
            id: id,
            index: index,
            incoming_edges: 0,
            state: ExpressionState::Verify
        }
    }

    // FIXME: This should never fail if validation is doing its job
    pub fn construct_units(&self, store: &Leap<DspOpcode>,
                           factory: &mut UnitFactory, units: &mut UnitMap,
                           parameters: &mut ParameterMap)
            -> ArtResult<()> {
        for opcode in try!(store.iter(self.index)) {
            if let &DspOpcode::Unit { unit_id, type_id, input_channels,
                                      output_channels } = opcode {
                let unit = factory.create((self.id, unit_id), type_id,
                                          input_channels, output_channels);
                unit.construct_parameters(parameters);
                units.insert((self.id, unit_id), unit);
            }
        }
        Ok(())
    }

    pub fn tick(&self, store: &Leap<DspOpcode>, stack: &mut ChannelStack,
                units: &mut UnitMap, adjuncts: &mut TickAdjuncts,
                constants: &Constants) -> ArtResult<()> {
        for opcode in try!(store.iter(self.index)) {
            match opcode {
                &DspOpcode::Unit { unit_id, .. } => {
                    let mut unit = try!(
                        units.get_mut(&(self.id, unit_id)).ok_or(
                            ArtError::UnitNotFound {
                                expression_id: self.id,
                                unit_id: unit_id
                            }
                        )
                    );
                    try!(
                        unit.tick(stack, adjuncts,
                                  constants)
                    );
                },
                &DspOpcode::Add { channels, rate } => {
                    try!(operators::add(stack, channels, rate, constants))
                },
                &DspOpcode::Multiply { channels, rate } => {
                    try!(operators::multiply(stack, channels, rate, constants))
                }
            }
        }
        Ok(())
    }

    pub fn validate(&self, store: &Leap<DspOpcode>,
                stack_record: &mut Vec<StackRecord>,
                unit_factory: &UnitFactory) -> ArtResult<()> {
        ExpressionValidator::validate(self.index, store, stack_record,
                                      unit_factory)
    }
}

