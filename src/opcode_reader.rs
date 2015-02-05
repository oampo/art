use std::old_io::{IoError, IoErrorKind, BufReader};
use std::num::FromPrimitive;

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
            ControlOpcodeType::Play => {
                self.read_play()
            }
        }
    }

    fn read_dsp_opcode_parameters(&mut self, opcode_type: DspOpcodeType)
            -> Result<DspOpcode, IoError> {
        match opcode_type {
            DspOpcodeType::Unit => {
                self.read_unit()
            },
            DspOpcodeType::Parameter => {
                self.read_parameter()
            },
            DspOpcodeType::Sample => {
                self.read_sample()
            }
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

        /*
        let mut opcodes = Vec::with_capacity(num_opcodes as usize);

        for _ in range(0, num_opcodes) {
            let opcode = try!(self.read_dsp_opcode());
            opcodes.push(opcode);
        }
        */

        Ok(
            ControlOpcode::AddExpression {
                expression_id: expression_id,
                num_opcodes: num_opcodes
            }
        )
    }

    fn read_play(&mut self) -> Result<ControlOpcode, IoError> {
        let expression_id = try!(self.read_be_u32());
        Ok(
            ControlOpcode::Play {
                expression_id: expression_id
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

    fn read_parameter(&mut self) -> Result<DspOpcode, IoError> {
        let expression_id = try!(self.read_be_u32());
        let unit_id = try!(self.read_be_u32());
        let parameter_id = try!(self.read_be_u32());
        Ok(
            DspOpcode::Parameter {
                expression_id: expression_id,
                unit_id: unit_id,
                parameter_id: parameter_id
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
