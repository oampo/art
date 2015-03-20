use std::io::{self, Read, Cursor};
use std::num::FromPrimitive;

use byteorder::{ReadBytesExt, BigEndian};

use types::Rate;
use opcode::{ControlOpcodeType, DspOpcodeType, ControlOpcode, DspOpcode};

pub trait OpcodeReader: Read {
    fn read_control_opcode(&mut self) -> Result<ControlOpcode, io::Error> {
        let opcode_type = try!(self.read_control_opcode_type());
        self.read_control_opcode_parameters(opcode_type)
    }

    fn read_dsp_opcode(&mut self) -> Result<DspOpcode, io::Error> {
        let opcode_type = try!(self.read_dsp_opcode_type());
        self.read_dsp_opcode_parameters(opcode_type)
    }

    fn read_control_opcode_type(&mut self)
            -> Result<ControlOpcodeType, io::Error> {
        let opcode_value = try!(self.read_u32::<BigEndian>());
        FromPrimitive::from_u32(opcode_value).ok_or(
            io::Error::new(io::ErrorKind::InvalidInput, "Unknown opcode",
                           None)
        )
    }
    fn read_dsp_opcode_type(&mut self) -> Result<DspOpcodeType, io::Error> {
        let opcode_value = try!(self.read_u32::<BigEndian>());
        FromPrimitive::from_u32(opcode_value).ok_or(
            io::Error::new(io::ErrorKind::InvalidInput, "Unknown opcode",
                           None)
        )
    }

    fn read_control_opcode_parameters(&mut self,
                                      opcode_type: ControlOpcodeType)
            -> Result<ControlOpcode, io::Error> {
        match opcode_type {
            ControlOpcodeType::SetParameter => {
                self.read_set_parameter()
            },
            ControlOpcodeType::AddExpression => {
                self.read_add_expression()
            },
            ControlOpcodeType::RemoveExpression => {
                self.read_remove_expression()
            },
            ControlOpcodeType::AddEdge => {
                self.read_add_edge()
            }
        }
    }

    fn read_dsp_opcode_parameters(&mut self, opcode_type: DspOpcodeType)
            -> Result<DspOpcode, io::Error> {
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

    fn read_set_parameter(&mut self) -> Result<ControlOpcode, io::Error> {
        let expression_id = try!(self.read_u32::<BigEndian>());
        let unit_id = try!(self.read_u32::<BigEndian>());
        let parameter_id = try!(self.read_u32::<BigEndian>());
        let value = try!(self.read_f32::<BigEndian>());
        Ok(
            ControlOpcode::SetParameter {
                expression_id: expression_id,
                unit_id: unit_id,
                parameter_id: parameter_id,
                value: value
            }
        )
    }

    fn read_add_expression(&mut self) -> Result<ControlOpcode, io::Error> {
        let expression_id = try!(self.read_u32::<BigEndian>());
        let num_opcodes = try!(self.read_u32::<BigEndian>());

        Ok(
            ControlOpcode::AddExpression {
                expression_id: expression_id,
                num_opcodes: num_opcodes
            }
        )
    }

    fn read_remove_expression(&mut self) -> Result<ControlOpcode, io::Error> {
        let expression_id = try!(self.read_u32::<BigEndian>());

        Ok(
            ControlOpcode::RemoveExpression {
                expression_id: expression_id
            }
        )
    }

    fn read_add_edge(&mut self) -> Result<ControlOpcode, io::Error> {
        let from = try!(self.read_u32::<BigEndian>());
        let to = try!(self.read_u32::<BigEndian>());

        Ok(
            ControlOpcode::AddEdge {
                from: from,
                to: to
            }
        )
    }

    fn read_unit(&mut self) -> Result<DspOpcode, io::Error> {
        let unit_id = try!(self.read_u32::<BigEndian>());
        let type_id = try!(self.read_u32::<BigEndian>());
        let input_channels = try!(self.read_u32::<BigEndian>());
        let output_channels = try!(self.read_u32::<BigEndian>());
        Ok(
            DspOpcode::Unit {
                unit_id: unit_id,
                type_id: type_id,
                input_channels: input_channels,
                output_channels: output_channels
            }
        )
    }

    fn read_operator(&mut self) -> Result<(u32, Rate), io::Error> {
        let channels = try!(self.read_u32::<BigEndian>());
        let raw_rate = try!(self.read_u32::<BigEndian>());
        let rate = try!(
            FromPrimitive::from_u32(raw_rate).ok_or(
                io::Error::new(io::ErrorKind::InvalidInput, "Unknown rate",
                               None)
            )
        );
        Ok((channels, rate))
    }

    fn read_add(&mut self) -> Result<DspOpcode, io::Error> {
        let (channels, rate) = try!(self.read_operator());
        Ok(
            DspOpcode::Add {
                channels: channels,
                rate: rate
            }
        )
    }

    fn read_multiply(&mut self) -> Result<DspOpcode, io::Error> {
        let (channels, rate) = try!(self.read_operator());
        Ok(
            DspOpcode::Multiply {
                channels: channels,
                rate: rate
            }
        )
    }
}

impl<'a> OpcodeReader for Cursor<&'a [u8]> {
}

impl<'a> OpcodeReader for Cursor<&'a mut [u8]> {
}
