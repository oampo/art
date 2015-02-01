#[derive(Copy)]
pub struct Sizes {
    pub block_size: usize,
    pub block_size_inverse: f32
}

#[derive(Copy)]
pub struct Rates {
    pub audio_rate: f32,
    pub audio_rate_inverse: f32
}

#[derive(Copy)]
pub struct Constants {
    pub rates: Rates,
    pub sizes: Sizes
}
