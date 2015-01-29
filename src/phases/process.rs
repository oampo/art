use std::io::BufReader;

use types::ArtResult;
use errors::ArtError;

use vm_inner::VMInner;
use opcode::{Opcode, ControlOpcode};
use opcode_reader::OpcodeReader;

use instructions::control::add_expression::AddExpression;
use instructions::control::set_parameter::SetParameter;

pub trait Process {
    fn process(&mut self);
    fn process_byte_code(&mut self, byte_code: &[u8]) -> ArtResult<()>;
    fn process_opcode(&mut self, reader: &mut BufReader) -> ArtResult<()>;
}

impl Process for VMInner {
    fn process(&mut self) {
        debug!("Starting process phase");
        loop {
            let result = self.input_channel.try_recv();
            match result {
                Ok(byte_code) => {
                    let result = self.process_byte_code(byte_code.as_slice());
                    result.unwrap_or_else(|error| error!("{}", error));
                },
                Err(_) => { return; }
            }
        }
    }

    fn process_byte_code(&mut self, byte_code: &[u8]) -> ArtResult<()> {
        let mut reader = BufReader::new(byte_code);
        while !reader.eof() {
            try!(self.process_opcode(&mut reader));
        }
        Ok(())
    }

    fn process_opcode(&mut self, reader: &mut BufReader) -> ArtResult<()> {
        let opcode = try!(
            reader.read_control_opcode()
        );

        match opcode {
            ControlOpcode::AddExpression { expression_id, opcodes } => {
                self.add_expression(expression_id, opcodes)
            },
            ControlOpcode::SetParameter { expression_id, unit_id,
                                          parameter_id, value } => {
                self.set_parameter((expression_id, unit_id, parameter_id),
                                   value)
            },
            _ => Err(ArtError::UnimplementedOpcode {
                opcode: Opcode::Control(opcode)
            })
        }
    }
}
