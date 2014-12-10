use types::ArtResult;
use errors::InvalidByteCodeError;
use vm::UnitMap;
use instruction::InstructionBox;
use instructions::unit_instruction::UnitInstruction;
use opcode::Opcode;

pub type Block = Vec<f32>;
pub type BlockStack = Vec<Block>;

pub struct Expression {
    instructions: Vec<InstructionBox>,
    channels: Vec<f32>,
    channel_stack: Vec<u32>,
    channel_pointer: u32

}

impl Expression {
    pub fn new(byte_code: &[u32]) -> ArtResult<Expression> {
        let instructions = try!(Expression::parse_bytecode(byte_code));
        Ok(
            Expression {
                instructions: instructions,
                channels: Vec::new(),
                channel_stack: Vec::new(),
                channel_pointer: 0
            }
        )
    }

    fn parse_bytecode(byte_code: &[u32]) -> ArtResult<Vec<InstructionBox>> {
        let mut instructions: Vec<InstructionBox> = Vec::new();
        let mut remaining_byte_code = byte_code;
        loop {
            match remaining_byte_code {
                [opcode, unit_id, rest..]
                        if opcode == Opcode::Unit as u32 => {
                    instructions.push(box UnitInstruction::new(unit_id));
                    remaining_byte_code = rest;
                },
                _ => break
            }
        }

        if !remaining_byte_code.is_empty() {
            return Err(InvalidByteCodeError::new());
        }
        Ok(instructions)
    }

    pub fn execute(&mut self, units: &mut UnitMap) -> ArtResult<()> {
        for instruction in self.instructions.iter_mut() {
            try!(instruction.execute(&mut self.channels, &mut self.channel_stack, &mut self.channel_pointer, units));
        }
        Ok(())
    }
}
