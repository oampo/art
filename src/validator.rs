use types::{ArtResult, StackRecord, Rate};
use errors::ArtError;
use opcode::DspOpcode;
use unit::UnitDefinition;
use unit_factory::UnitFactory;
use leap::Leap;

pub struct ExpressionValidator;

impl ExpressionValidator {
    pub fn validate(index: usize, store: &Leap<DspOpcode>,
                    stack_record: &mut Vec<StackRecord>,
                    unit_factory: &UnitFactory) -> ArtResult<()> {
        for opcode in try!(store.iter(index)) {
            match opcode {
                &DspOpcode::Unit { type_id, input_channels,
                                   output_channels, .. } => {
                    try!(
                        UnitValidator::validate_type(type_id, unit_factory)
                    );

                    let definition = unit_factory.get_definition(type_id);
                    try!(
                        UnitValidator::validate_stack(
                            input_channels, output_channels, definition,
                            stack_record
                        )
                    );
                },
                &DspOpcode::Add { channels, rate } |
                &DspOpcode::Multiply { channels, rate } => {
                    try!(
                        OperatorValidator::validate_stack(channels, rate,
                                                          stack_record)
                    );
                }
            }
        }
        Ok(())
    }
}

struct UnitValidator;

impl UnitValidator {
    fn validate_type(type_id: u32, unit_factory: &UnitFactory)
            -> ArtResult<()> {
        if !unit_factory.is_registered(type_id) {
            return Err(
                ArtError::UndefinedUnit { type_id: type_id }
            );
        }
        Ok(())
    }

    fn validate_stack(input_channels: u32, output_channels: u32,
                      definition: &UnitDefinition,
                      stack_record: &mut Vec<StackRecord>) -> ArtResult<()> {
        if input_channels != 0 {
            if stack_record.len() == 0 {
                return Err(ArtError::StackUnderflow);
            }

            let record = stack_record.pop().unwrap();

            try!(
                UnitValidator::validate_channels(input_channels,
                                                 record.channels)
            );
            try!(
                UnitValidator::validate_rate(definition.input_rate,
                                             record.rate)
            );
        }

        if output_channels != 0 {
            if stack_record.len() == stack_record.capacity() {
                return Err(ArtError::StackOverflow);
            }

            stack_record.push(
                StackRecord {
                    channels: output_channels,
                    rate: definition.output_rate
                }
            );
        }
        Ok(())
    }

    fn validate_channels(channels_a: u32, channels_b: u32) -> ArtResult<()> {
        if channels_a != channels_b {
            return Err(
                ArtError::ChannelMismatch {
                   expected: channels_a,
                   actual: channels_b
                }
            );
        }
        Ok(())
    }

    fn validate_rate(rate_a: Rate, rate_b: Rate) -> ArtResult<()> {
        if rate_a != rate_b {
            return Err(
                ArtError::RateMismatch {
                    expected: rate_a,
                    actual: rate_b
                }
            );
        }
        Ok(())
    }
}

struct OperatorValidator;

impl OperatorValidator {
    fn validate_stack(channels: u32, rate: Rate,
                      stack_record: &mut Vec<StackRecord>)
            -> ArtResult<()> {
        if channels != 0 {
            if stack_record.len() < 2 {
                return Err(ArtError::StackUnderflow);
            }

            let record_a = stack_record.pop().unwrap();
            let record_b = stack_record.pop().unwrap();

            try!(
                OperatorValidator::validate_channels(
                    channels, record_a.channels, record_b.channels
                )
            );
            try!(
                OperatorValidator::validate_rate(
                    rate, record_a.rate, record_b.rate
                )
            );

            // No need to check that there is space, because we've just popped
            // two items
            stack_record.push(
                StackRecord {
                    channels: channels,
                    rate: rate
                }
            );
        }
        Ok(())
    }

    fn validate_channels(channels: u32, channels_a: u32, channels_b: u32)
            -> ArtResult<()> {
        if channels != channels_a {
            return Err(
                ArtError::ChannelMismatch {
                    expected: channels,
                    actual: channels_a
                }
            );
        }

        if channels != channels_b {
            return Err(
                ArtError::ChannelMismatch {
                    expected: channels,
                    actual: channels_b
                }
            );
        }
        Ok(())
    }

    fn validate_rate(rate: Rate, rate_a: Rate, rate_b: Rate) -> ArtResult<()> {
        if rate != rate_a {
            return Err(
                ArtError::RateMismatch {
                    expected: rate,
                    actual: rate_a
                }
            );
        }

        if rate != rate_b {
            return Err(
                ArtError::RateMismatch {
                    expected: rate,
                    actual: rate_b
                }
            );
        }
        Ok(())
    }
}
