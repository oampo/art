use std::io::{IoError, IoErrorKind, BufReader};
use std::num::FromPrimitive;

use opcode::{OpcodeType, ControlOpcode, DspOpcode};

pub trait OpcodeReader: Reader {
    fn read_control_opcode(&mut self) -> Result<ControlOpcode, IoError> {
        let opcode_type = try!(self.read_opcode_type());
        self.read_control_opcode_parameters(opcode_type)
    }

    fn read_opcode_type(&mut self) -> Result<OpcodeType, IoError> {
        let opcode_value = try!(self.read_be_u32());
        Ok(
            FromPrimitive::from_u32(opcode_value).unwrap_or(
                OpcodeType::Unknown
            )
        )
    }

    fn read_control_opcode_parameters(&mut self, opcode_type: OpcodeType)
            -> Result<ControlOpcode, IoError> {
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
            _ => {
                Ok(ControlOpcode::Unknown)
            }
        }
    }

    fn read_dsp_opcode_parameters(&mut self, opcode_type: OpcodeType)
            -> Result<DspOpcode, IoError> {
        match opcode_type {
            OpcodeType::Unit => {
                self.read_unit()
            },
            OpcodeType::Parameter => {
                self.read_parameter()
            },
            OpcodeType::Sample => {
                self.read_sample()
            },
            OpcodeType::Dac => {
                Ok(DspOpcode::Dac)
            },
            OpcodeType::Adc => {
                Ok(DspOpcode::Adc)
            },
            _ => {
                Ok(DspOpcode::Unknown)
            }
        }
    }

    fn read_create_unit(&mut self) -> Result<ControlOpcode, IoError> {
        let id = try!(self.read_be_u32());
        let type_id = try!(self.read_be_u32());
        let input_channels = try!(self.read_be_u32());
        let output_channels = try!(self.read_be_u32());
        Ok(
            ControlOpcode::CreateUnit {
                id: id,
                type_id: type_id,
                input_channels: input_channels,
                output_channels: output_channels
            }
        )
    }

    fn read_set_parameter(&mut self) -> Result<ControlOpcode, IoError> {
        let unit_id = try!(self.read_be_u32());
        let id = try!(self.read_be_u32());
        let value = try!(self.read_be_f32());
        Ok(
            ControlOpcode::SetParameter {
                unit_id: unit_id,
                id: id,
                value: value
            }
        )
    }

    fn read_expression(&mut self) -> Result<ControlOpcode, IoError> {
        let id = try!(self.read_be_u32());
        let mut opcodes = Vec::new();

        loop {
            // TODO: Can this logic be simplified?
            let opcode_type_result = self.read_opcode_type();

            match opcode_type_result {
                Ok(opcode_type) => {
                    let opcode = try!(self.read_dsp_opcode_parameters(opcode_type));
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
            ControlOpcode::Expression {
                id: id,
                opcodes: opcodes
            }
        )
    }

    fn read_play(&mut self) -> Result<ControlOpcode, IoError> {
        let id = try!(self.read_be_u32());
        Ok(
            ControlOpcode::Play {
                id: id
            }
        )
    }

    fn read_unit(&mut self) -> Result<DspOpcode, IoError> {
        let id = try!(self.read_be_u32());
        Ok(
            DspOpcode::Unit {
                id: id
            }
        )
    }

    fn read_parameter(&mut self) -> Result<DspOpcode, IoError> {
        let unit_id = try!(self.read_be_u32());
        let id = try!(self.read_be_u32());
        Ok(
            DspOpcode::Parameter {
                unit_id: unit_id,
                id: id
            }
        )
    }

    fn read_sample(&mut self) -> Result<DspOpcode, IoError> {
        let value = try!(self.read_be_f32());
        Ok(
            DspOpcode::Sample {
                value: value
            }
        )
    }
}

impl<'a> OpcodeReader for BufReader<'a> {
}
