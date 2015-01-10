#[derive(Copy)]
pub enum DeviceId {
    Id(u32),
    Default
}

impl DeviceId {
    pub fn from_option(id: Option<u32>) -> DeviceId {
        id.map_or(DeviceId::Default, |id| DeviceId::Id(id))
    }
}
