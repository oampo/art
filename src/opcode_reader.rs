use std::io::{IoError, IoErrorKind, BufReader};
use std::num::FromPrimitive;

use opcode::{OpcodeType, Opcode};

pub trait OpcodeReader: Reader {
    fn read_opcode(&mut self) -> Result<Opcode, IoError> {
        let opcode_type = try!(self.read_opcode_type());
        self.read_opcode_parameters(opcode_type)
    }

    fn read_opcode_type(&mut self) -> Result<OpcodeType, IoError> {
        let opcode_value = try!(self.read_be_u32());
        Ok(
            FromPrimitive::from_u32(opcode_value).unwrap_or(
                OpcodeType::Unknown
            )
        )
    }

    fn read_opcode_parameters(&mut self, opcode_type: OpcodeType)
            -> Result<Opcode, IoError> {
        match opcode_type {
            OpcodeType::CreateUnit => {
                self.read_create_unit()
            },
            OpcodeType::SetParameter => {
                self.read_set_parameter()
            },
            OpcodeType::Expression => {
                self.read_expression()
            },
            OpcodeType::Play => {
                self.read_play()
            },
            OpcodeType::Unit => {
                self.read_unit()
            },
            OpcodeType::Parameter => {
                self.read_parameter()
            },
            OpcodeType::Sample => {
                self.read_sample()
            },
            OpcodeType::DAC => {
                Ok(Opcode::DAC)
            }
            OpcodeType::ADC => {
                Ok(Opcode::ADC)
            }
            OpcodeType::Unknown => {
                Ok(Opcode::Unknown)
            }
        }
    }

    fn read_create_unit(&mut self) -> Result<Opcode, IoError> {
        let id = try!(self.read_be_u32());
        let type_id = try!(self.read_be_u32());
        let input_channels = try!(self.read_be_u32());
        let output_channels = try!(self.read_be_u32());
        Ok(
            Opcode::CreateUnit {
                id: id,
                type_id: type_id,
                input_channels: input_channels,
                output_channels: output_channels
            }
        )
    }

    fn read_set_parameter(&mut self) -> Result<Opcode, IoError> {
        let unit_id = try!(self.read_be_u32());
        let parameter_id = try!(self.read_be_u32());
        let value = try!(self.read_be_f32());
        Ok(
            Opcode::SetParameter {
                unit_id: unit_id,
                parameter_id: parameter_id,
                value: value
            }
        )
    }

    fn read_expression(&mut self) -> Result<Opcode, IoError> {
        let id = try!(self.read_be_u32());
        let mut opcodes = Vec::new();

        loop {
            // TODO: Can this logic be simplified?
            let opcode_type_result = self.read_opcode_type();

            match opcode_type_result {
                Ok(opcode_type) => {
                    let opcode = try!(self.read_opcode_parameters(opcode_type));
                    opcodes.push(opcode);
                },
                Err(error) => {
                    match error.kind {
                        IoErrorKind::EndOfFile => break,
                        _ => return Err(error)
                    }
                }
            }
        }

        Ok(
            Opcode::Expression {
                id: id,
                opcodes: opcodes
            }
        )
    }

    fn read_play(&mut self) -> Result<Opcode, IoError> {
        let id = try!(self.read_be_u32());
        Ok(
            Opcode::Play {
                id: id
            }
        )
    }

    fn read_unit(&mut self) -> Result<Opcode, IoError> {
        let id = try!(self.read_be_u32());
        Ok(
            Opcode::Unit {
                id: id
            }
        )
    }

    fn read_parameter(&mut self) -> Result<Opcode, IoError> {
        let unit_id = try!(self.read_be_u32());
        let parameter_id = try!(self.read_be_u32());
        Ok(
            Opcode::Parameter {
                unit_id: unit_id,
                parameter_id: parameter_id
            }
        )
    }

    fn read_sample(&mut self) -> Result<Opcode, IoError> {
        let value = try!(self.read_be_f32());
        Ok(
            Opcode::Sample {
                value: value
            }
        )
    }
}

impl<'a> OpcodeReader for BufReader<'a> {
}
