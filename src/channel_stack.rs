use types::ArtResult;
use errors::{StackFullError, InvalidStackError};
use sizes::BLOCK_SIZE;

#[derive(Show)]
pub struct ChannelStack {
    pub data: Vec<f32>,
    pub stack: Vec<u32>,
    pub position: usize
}

impl ChannelStack {
    pub fn new(size: u32) -> ChannelStack {
        let mut stack = ChannelStack {
            data: Vec::with_capacity(size as usize * BLOCK_SIZE),
            stack: Vec::with_capacity(size as usize),
            position: 0
        };

        stack.data.resize(size as usize * BLOCK_SIZE, 0f32);
        stack
    }

    pub fn push(&mut self, channels: u32) -> ArtResult<()> {
        let new_position = self.position + channels as usize * BLOCK_SIZE;

        if new_position > self.data.len() {
            return Err(StackFullError::new());
        }

        // Check for full data stack should prevent needing to reallocate
        // the channel history stack
        assert!(self.stack.len() < self.stack.capacity());
        self.stack.push(channels);
        self.position = new_position;
        Ok(())
    }

    pub fn pop(&mut self) -> ArtResult<u32> {
        let channels = try!(
            self.stack.pop().ok_or(InvalidStackError::new())
        );

        self.position -= channels as usize * BLOCK_SIZE;
        Ok(channels)
    }

    pub fn pop_expect(&mut self, expected_channels: u32) -> ArtResult<u32> {
        let channels = try!(self.pop());

        if channels == expected_channels {
            return Ok(channels);
        }

        Err(InvalidStackError::new())
    }
}
