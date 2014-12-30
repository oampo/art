pub struct ChannelStack {
    pub data: Vec<f32>,
    pub stack: Vec<u32>,
    pub position: u32
}

impl ChannelStack {
    pub fn new() -> ChannelStack {
        ChannelStack {
            data: Vec::new(),
            stack: Vec::new(),
            position: 0
        }
    }
}
