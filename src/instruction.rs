use types::ArtResult;
use vm::UnitMap;

pub trait Instruction {
    fn execute(&mut self, channels: &mut Vec<f32>,
               channel_stack: &mut Vec<u32>, channel_pointer: &mut u32,
               units: &mut UnitMap)
            -> ArtResult<()>;
}

pub type InstructionBox = Box<Instruction + 'static>;
