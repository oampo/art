use std::old_io::{IoError, IoErrorKind, BufReader};
use std::num::FromPrimitive;

use types::Rate;
use opcode::{ControlOpcodeType, DspOpcodeType, ControlOpcode, DspOpcode};

pub trait OpcodeReader: Reader {
    fn read_control_opcode(&mut self) -> Result<ControlOpcode, IoError> {
        let opcode_type = try!(self.read_control_opcode_type());
        self.read_control_opcode_parameters(opcode_type)
    }

    fn read_dsp_opcode(&mut self) -> Result<DspOpcode, IoError> {
        let opcode_type = try!(self.read_dsp_opcode_type());
        self.read_dsp_opcode_parameters(opcode_type)
    }

    fn read_control_opcode_type(&mut self)
            -> Result<ControlOpcodeType, IoError> {
        let opcode_value = try!(self.read_be_u32());
        FromPrimitive::from_u32(opcode_value).ok_or(
            IoError {
                kind: IoErrorKind::InvalidInput,
                desc: "Unknown opcode",
                detail: None
            }
        )
    }
    fn read_dsp_opcode_type(&mut self) -> Result<DspOpcodeType, IoError> {
        let opcode_value = try!(self.read_be_u32());
        FromPrimitive::from_u32(opcode_value).ok_or(
            IoError {
                kind: IoErrorKind::InvalidInput,
                desc: "Unknown opcode",
                detail: None
            }
        )
    }

    fn read_control_opcode_parameters(&mut self,
                                      opcode_type: ControlOpcodeType)
            -> Result<ControlOpcode, IoError> {
        match opcode_type {
            ControlOpcodeType::SetParameter => {
                self.read_set_parameter()
            },
            ControlOpcodeType::AddExpression => {
                self.read_expression()
            },
            ControlOpcodeType::AddEdge => {
                self.read_add_edge()
            }
        }
    }

    fn read_dsp_opcode_parameters(&mut self, opcode_type: DspOpcodeType)
            -> Result<DspOpcode, IoError> {
        match opcode_type {
            DspOpcodeType::Unit => {
                self.read_unit()
            },
            DspOpcodeType::Add => {
                self.read_add()
            },
            DspOpcodeType::Multiply=> {
                self.read_multiply()
            },
        }
    }

    fn read_set_parameter(&mut self) -> Result<ControlOpcode, IoError> {
        let expression_id = try!(self.read_be_u32());
        let unit_id = try!(self.read_be_u32());
        let parameter_id = try!(self.read_be_u32());
        let value = try!(self.read_be_f32());
        Ok(
            ControlOpcode::SetParameter {
                expression_id: expression_id,
                unit_id: unit_id,
                parameter_id: parameter_id,
                value: value
            }
        )
    }

    fn read_expression(&mut self) -> Result<ControlOpcode, IoError> {
        let expression_id = try!(self.read_be_u32());
        let num_opcodes = try!(self.read_be_u32());

        Ok(
            ControlOpcode::AddExpression {
                expression_id: expression_id,
                num_opcodes: num_opcodes
            }
        )
    }

    fn read_add_edge(&mut self) -> Result<ControlOpcode, IoError> {
        let from = try!(self.read_be_u32());
        let to = try!(self.read_be_u32());

        Ok(
            ControlOpcode::AddEdge {
                from: from,
                to: to
            }
        )
    }

    fn read_unit(&mut self) -> Result<DspOpcode, IoError> {
        let unit_id = try!(self.read_be_u32());
        let type_id = try!(self.read_be_u32());
        let input_channels = try!(self.read_be_u32());
        let output_channels = try!(self.read_be_u32());
        Ok(
            DspOpcode::Unit {
                unit_id: unit_id,
                type_id: type_id,
                input_channels: input_channels,
                output_channels: output_channels
            }
        )
    }

    fn read_operator(&mut self) -> Result<(u32, Rate), IoError> {
        let channels = try!(self.read_be_u32());
        let raw_rate = try!(self.read_be_u32());
        let rate = try!(
            FromPrimitive::from_u32(raw_rate).ok_or(
                IoError {
                    kind: IoErrorKind::InvalidInput,
                    desc: "Unknown rate",
                    detail: None
                }
            )
        );
        Ok((channels, rate))
    }

    fn read_add(&mut self) -> Result<DspOpcode, IoError> {
        let (channels, rate) = try!(self.read_operator());
        Ok(
            DspOpcode::Add {
                channels: channels,
                rate: rate
            }
        )
    }

    fn read_multiply(&mut self) -> Result<DspOpcode, IoError> {
        let (channels, rate) = try!(self.read_operator());
        Ok(
            DspOpcode::Multiply {
                channels: channels,
                rate: rate
            }
        )
    }
}

impl<'a> OpcodeReader for BufReader<'a> {
}
