use std::old_io::BufReader;

use types::ArtResult;

use leap::Leap;
use opcode_reader::OpcodeReader;
use opcode::DspOpcode;

pub trait ExpressionStore {
    fn push_from_reader(&mut self, num_opcodes: u32,
                            reader: &mut BufReader) -> ArtResult<usize>;

    fn push_opcode_from_reader(&mut self, reader: &mut BufReader)
            -> ArtResult<()>;
}

impl ExpressionStore for Leap<DspOpcode> {
    fn push_from_reader(&mut self, num_opcodes: u32,
                            reader: &mut BufReader) -> ArtResult<usize> {
        let start = try!(self.alloc(num_opcodes as usize));
        for _ in range(0, num_opcodes) {
            let result = self.push_opcode_from_reader(reader);

            if result.is_err() {
                let _ = self.free(start);
                return Err(result.err().unwrap());
            }
        }
        Ok(start)
    }

    fn push_opcode_from_reader(&mut self, reader: &mut BufReader)
            -> ArtResult<()> {
       let opcode = try!(reader.read_dsp_opcode());
       try!(self.push(opcode));
       Ok(())
    }
}

