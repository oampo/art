use types::{ArtResult, Rate};
use constants::Constants;
use channel_stack::ChannelStack;

pub fn add(stack: &mut ChannelStack, channels: u32, rate: Rate,
           constants: &Constants) -> ArtResult<()> {
    let samples = match rate {
        Rate::Audio => channels as usize * constants.block_size,
        Rate::Control => channels as usize
    };

    let index_b = try!(stack.pop(samples));
    let index_a = try!(stack.pop(samples));
    try!(stack.push(samples));

    for i in range(0, samples) {
        stack.data[index_a + i] += stack.data[index_b + i];
    }
    Ok(())
}

pub fn multiply(stack: &mut ChannelStack, channels: u32, rate: Rate,
                constants: &Constants) -> ArtResult<()> {
    let samples = match rate {
        Rate::Audio => channels as usize * constants.block_size,
        Rate::Control => channels as usize
    };

    let index_b = try!(stack.pop(samples));
    let index_a = try!(stack.pop(samples));
    try!(stack.push(samples));

    for i in range(0, samples) {
        stack.data[index_a + i] *= stack.data[index_b + i];
    }
    Ok(())
}

