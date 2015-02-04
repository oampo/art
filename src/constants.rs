#[derive(Copy)]
pub struct Constants {
    pub input_channels: u32,
    pub output_channels: u32,
    pub block_size: usize,
    pub block_size_inverse: f32,
    pub audio_rate: f32,
    pub audio_rate_inverse: f32
}
