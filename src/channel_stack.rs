use types::ArtResult;
use errors::InvalidStackError;
use sizes::BLOCK_SIZE;

pub struct ChannelStack {
    pub data: Vec<f32>,
    pub stack: Vec<u32>,
    pub position: uint
}

impl ChannelStack {
    pub fn new() -> ChannelStack {
        ChannelStack {
            data: Vec::new(),
            stack: Vec::new(),
            position: 0
        }
    }

    pub fn push(&mut self, channels: u32) {
        let current_length = self.data.len();
        let new_position = self.position + channels as uint * BLOCK_SIZE;
        if current_length < new_position {
            // Not enough channels allocated, so grow the vector
            self.data.resize(new_position, 0f32);
        }
        self.stack.push(channels);
        self.position = new_position;
    }

    pub fn pop(&mut self) -> ArtResult<u32> {
        let channels = try!(
            self.stack.pop().ok_or(InvalidStackError::new())
        );

        self.position -= channels as uint * BLOCK_SIZE;
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
