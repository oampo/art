use std::mem;

use types::ArtResult;
use errors::ArtError;

#[derive(Debug)]
pub struct ChannelStack<'a> {
    data: &'a mut[f32],
    channel_size: usize,
    position: u32,
    capacity: usize
}

impl<'a> ChannelStack<'a> {
    pub fn new(data: &'a mut[f32], channel_size: usize) -> ChannelStack<'a> {
        let len = data.len();
        ChannelStack {
            data: data,
            channel_size: channel_size,
            position: 0,
            capacity: len / channel_size,
        }
    }

    pub fn push(&mut self, channels: u32) -> ArtResult<u32> {
        let mut position = self.position + channels;

        if position as usize > self.capacity {
            return Err(ArtError::StackOverflow);
        }

        mem::swap(&mut position, &mut self.position);
        Ok(position)
    }

    pub fn pop(&mut self, channels: u32) -> ArtResult<u32> {
        if channels > self.position {
            return Err(ArtError::StackUnderflow);
        }

        self.position -= channels;
        Ok(self.position)
    }

    pub fn read(&self, index: u32, values: &mut[f32]) -> ArtResult<()> {
        if index as usize + values.len() / self.channel_size > self.capacity {
            return Err(ArtError::StackOverflow);
        }

        values.clone_from_slice(&self.data[index as usize * self.channel_size..]);
        Ok(())
    }

    pub fn write(&mut self, index: u32, values: &[f32]) -> ArtResult<()> {
        if index as usize + values.len() / self.channel_size > self.capacity {
            return Err(ArtError::StackOverflow);
        }

        (&mut self.data[index as usize * self.channel_size..]).clone_from_slice(values);
        Ok(())
    }

    pub fn add(&mut self, index: u32, values: &[f32]) -> ArtResult<()> {
        if index as usize + values.len() / self.channel_size > self.capacity {
            return Err(ArtError::StackOverflow);
        }

        let start = index as usize * self.channel_size;
        for i in range(0, values.len()) {
            self.data[start + i] += values[i];
        }
        Ok(())
    }


    pub fn get(&mut self, index: u32, channels: u32) -> ArtResult<&mut[f32]> {
        if (index + channels) as usize > self.capacity {
            return Err(ArtError::StackOverflow);
        }

        let start = index as usize * self.channel_size;
        let end = start + channels as usize * self.channel_size;

        Ok(&mut self.data[start..end])
    }

    pub fn split(&mut self, index: u32)
            -> (ChannelStack, ChannelStack) {
        let (left, right) = self.data.split_at_mut(
            index as usize * self.channel_size
        );

        let left_stack = ChannelStack::new(left, self.channel_size);
        let right_stack = ChannelStack::new(right, self.channel_size);
        return (left_stack, right_stack);
    }
}
