use types::ArtResult;

use leap::Leap;
use opcode_reader::OpcodeReader;
use opcode::DspOpcode;

pub trait ExpressionStore {
    fn push_from_reader<T>(&mut self, num_opcodes: usize,
                           reader: &mut T) -> ArtResult<usize>
            where T: OpcodeReader;

    fn push_opcode_from_reader<T>(&mut self, reader: &mut T)
            -> ArtResult<()> where T: OpcodeReader;
}

impl ExpressionStore for Leap<DspOpcode> {
    fn push_from_reader<T>(&mut self, num_opcodes: usize,
                           reader: &mut T) -> ArtResult<usize>
            where T: OpcodeReader {
        let start = self.tail;
        for i in range(0, num_opcodes) {
            let result = self.push_opcode_from_reader(reader);

            if result.is_err() {
                let _ = self.free(start, i);
                return Err(result.err().unwrap());
            }
        }
        Ok(start)
    }

    fn push_opcode_from_reader<T>(&mut self, reader: &mut T)
            -> ArtResult<()> where T: OpcodeReader {
       let opcode = try!(reader.read_dsp_opcode());
       try!(self.push(opcode));
       Ok(())
    }
}

