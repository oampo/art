use types::{ArtResult, UnitMap, ParameterMap, StackRecord};
use unit::TickAdjuncts;
use errors::ArtError;
use constants::Constants;
use opcode::{DspOpcode};
use unit_factory::UnitFactory;
use channel_stack::ChannelStack;
use leap::Leap;
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
        // TODO: Validate space for units and parameters
        let mut stack_pointer = 0us;
        for opcode in try!(store.iter(self.index)) {
            match opcode {
                &DspOpcode::Unit { type_id, input_channels,
                                   output_channels, .. } => {
                    if !unit_factory.is_registered(type_id) {
                        return Err(
                            ArtError::UndefinedUnit { type_id: type_id }
                        );
                    }

                    let definition = unit_factory.get_definition(type_id);

                    if input_channels != 0 {
                        if stack_pointer == 0 {
                            return Err(ArtError::StackUnderflow);
                        }

                        stack_pointer -= 1;
                        let record = &stack_record[stack_pointer];

                        if record.channels != input_channels {
                            return Err(
                                ArtError::ChannelMismatch {
                                   expected: input_channels,
                                    actual: record.channels
                                }
                            );
                        }

                        if record.rate != definition.input_rate {
                            return Err(
                                ArtError::RateMismatch {
                                    expected: definition.input_rate,
                                    actual: record.rate
                                }
                            );
                        }
                    }

                    if output_channels != 0 {
                        if stack_pointer >= stack_record.len() {
                            return Err(ArtError::StackOverflow);
                        }

                        let record = &mut stack_record[stack_pointer];
                        record.channels = output_channels;
                        record.rate = definition.output_rate;
                        stack_pointer += 1;
                    }
                },
                &DspOpcode::Add { channels, rate } |
                &DspOpcode::Multiply { channels, rate } => {
                    if channels != 0 {
                        if stack_pointer < 2 {
                            return Err(ArtError::StackUnderflow);
                        }

                        stack_pointer -= 1;
                        let record_a = &stack_record[stack_pointer];

                        stack_pointer -= 1;
                        let record_b = &stack_record[stack_pointer];

                        if record_a.channels != record_b.channels {
                            return Err(
                                ArtError::ChannelMismatch {
                                    expected: record_a.channels,
                                    actual: record_b.channels
                                }
                            );
                        }

                        if record_a.rate != rate {
                            return Err(
                                ArtError::RateMismatch {
                                    expected: record_a.rate,
                                    actual: rate
                                }
                            );
                        }

                        if record_a.rate != record_b.rate {
                            return Err(
                                ArtError::RateMismatch {
                                    expected: record_a.rate,
                                    actual: record_b.rate
                                }
                            );
                        }

                        // No need to set values in the new record, as the
                        // need to be the same as record_a
                        stack_pointer += 1;
                    }
                }
            }
        }
        Ok(())
    }
}

