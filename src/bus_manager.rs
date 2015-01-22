pub struct BusManager {
    data: Vec<f32>,
    channel_size: usize
}

impl BusManager {
    pub fn new(num_busses: usize, channel_size: usize) -> BusManager {
        BusManager {
            data: Vec::with_capacity(num_busses * channel_size),
            channel_size: channel_size
        }
    }

    pub fn reserve(&mut self, num_channels: usize) -> usize {
        // TODO: Should fail if beyond capacity
        let len = self.data.len();
        self.data.resize(len + num_channels as usize * self.channel_size, 0f32);
        len / self.channel_size
    }

    pub fn get(&self, index: usize, values: &mut[f32]) {
        values.clone_from_slice(self.data.slice_from(index * self.channel_size));
    }

    pub fn set(&mut self, index: usize, values: &[f32]) {
        self.data.slice_from_mut(index * self.channel_size).clone_from_slice(values);
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

