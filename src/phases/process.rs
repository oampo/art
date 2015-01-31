use std::io::BufReader;

use types::ArtResult;
use errors::ArtError;

use vm_inner::VmInner;
use opcode::{Opcode, ControlOpcode};
use opcode_reader::OpcodeReader;

use instructions::control::add_expression::AddExpression;
use instructions::control::set_parameter::SetParameter;

pub trait Process {
    fn process(&mut self);
    fn process_byte_code(&mut self, byte_code: &[u8]) -> ArtResult<()>;
    fn process_opcode(&mut self, reader: &mut BufReader) -> ArtResult<()>;
    fn process_expression(&mut self, reader: &mut BufReader,
                          num_opcodes: u32) -> ArtResult<()>;
}

impl Process for VmInner {
    fn process(&mut self) {
        debug!("Starting process phase");
        let result = self.input_channel.try_recv();
        if let Ok(byte_code) = result {
            let result = self.process_byte_code(byte_code.as_slice());
            result.unwrap_or_else(|error| error!("{}", error));
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
            ControlOpcode::AddExpression { expression_id, num_opcodes } => {
                let start = try!(
                    self.expression_list.push_start(num_opcodes as usize)
                );

                let result = self.process_expression(reader, num_opcodes);

                if result.is_err() {
                    try!(self.expression_list.remove(start));
                    return result;
                }

                self.add_expression(expression_id, start)
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

    fn process_expression(&mut self, reader: &mut BufReader,
                          num_opcodes: u32) -> ArtResult<()> {
        for _ in range(0, num_opcodes) {
            let opcode = try!(reader.read_dsp_opcode());
            try!(self.expression_list.push(opcode));
        }
        Ok(())
    }
}
