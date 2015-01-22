use std::io::BufReader;

use types::ArtResult;
use errors::{InvalidByteCodeError, UnimplementedOpcodeError};

use vm_inner::VMInner;
use opcode::{Opcode, ControlOpcode};
use opcode_reader::OpcodeReader;

use instructions::control::create_unit::CreateUnit;
use instructions::control::add_expression::AddExpression;
use instructions::control::set_parameter::SetParameter;

pub trait Process {
    fn process(&mut self);
    fn process_byte_code(&mut self, byte_code: &[u8]) -> ArtResult<()>;
}

impl Process for VMInner {
    fn process(&mut self) {
        debug!("Starting process phase");
        loop {
            let result = self.input_channel.try_recv();
            match result {
                Ok(byte_code) => {
                    let result = self.process_byte_code(byte_code.as_slice());
                    result.unwrap_or_else(|error| error!("{:?}", error));
                },
                Err(_) => { return; }
            }
        }
    }

    fn process_byte_code(&mut self, byte_code: &[u8]) -> ArtResult<()> {
        let mut reader = BufReader::new(byte_code);
        let opcode = try!(
            reader.read_control_opcode().map_err(|_|
                InvalidByteCodeError::new()
            )
        );

        match opcode {
            ControlOpcode::AddExpression { expression_id, opcodes } => {
                self.add_expression(expression_id, opcodes)
            },

            ControlOpcode::SetParameter { unit_id, parameter_id, value } => {
                self.set_parameter(unit_id, parameter_id, value)
            },

            ControlOpcode::CreateUnit { unit_id, type_id, input_channels,
                                        output_channels } => {
                self.create_unit(unit_id, type_id, input_channels,
                                 output_channels)
            },

            ControlOpcode::Unknown => Err(InvalidByteCodeError::new()),
            _ => Err(UnimplementedOpcodeError::new(Opcode::Control(opcode)))
        }
    }
}
