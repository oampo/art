use std::mem;

use types::ArtResult;
use errors::ArtError;

#[derive(Debug)]
pub struct ChannelStack<'a> {
    pub data: &'a mut[f32],
    position: usize
}

impl<'a> ChannelStack<'a> {
    pub fn new(data: &'a mut[f32]) -> ChannelStack<'a> {
        ChannelStack {
            data: data,
            position: 0
        }
    }

    pub fn push(&mut self, samples: usize) -> ArtResult<usize> {
        let mut position = self.position + samples;

        if position > self.data.len() {
            return Err(ArtError::StackOverflow);
        }

        mem::swap(&mut position, &mut self.position);
        Ok(position)
    }

    pub fn pop(&mut self, samples: usize) -> ArtResult<usize> {
        if samples > self.position {
            return Err(ArtError::StackUnderflow);
        }

        self.position -= samples;
        Ok(self.position)
    }

    pub fn read(&self, index: usize, values: &mut[f32]) {
        debug_assert!(index + values.len() <= self.data.len());
        values.clone_from_slice(&self.data[index..]);
    }

    pub fn write(&mut self, index: usize, values: &[f32]) {
        debug_assert!(index + values.len() <= self.data.len());
        (&mut self.data[index..]).clone_from_slice(values);
    }

    pub fn add(&mut self, index: usize, values: &[f32]) {
        debug_assert!(index + values.len() <= self.data.len());

        for i in range(0, values.len()) {
            self.data[index + i] += values[i];
        }
    }


    pub fn get_mut(&mut self, index: usize, samples: usize) -> &mut [f32] {
        debug_assert!(index + samples <= self.data.len());
        let end = index + samples;
        &mut self.data[index..end]
    }

    pub fn zero(&mut self, index: usize, samples: usize) {
        debug_assert!(index + samples <= self.data.len());
        for i in range(index, index + samples) {
            self.data[i] = 0f32;
        }
    }

    pub fn split_at_mut(&mut self, index: usize)
            -> (ChannelStack, ChannelStack) {
        debug_assert!(index <= self.data.len());
        let (left, right) = self.data.split_at_mut(index);

        let left_stack = ChannelStack::new(left);
        let right_stack = ChannelStack::new(right);
        return (left_stack, right_stack);
    }
}
