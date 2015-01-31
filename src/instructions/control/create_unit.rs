use types::ArtResult;
use vm_inner::VmInner;

pub trait CreateUnit {
    fn create_unit(&mut self, id: (u32, u32), type_id: u32, input_channels: u32,
                   output_channels: u32) -> ArtResult<()>;
}

impl CreateUnit for VmInner {
    fn create_unit(&mut self, id: (u32, u32), type_id: u32, input_channels: u32,
                   output_channels: u32) -> ArtResult<()> {
        let (eid, uid) = id;
        debug!("Creating unit: expression_id={}, unit_id={}, type_id={},
                input_channels={}, output_channel={}", eid, uid, type_id,
                input_channels, output_channels);
        let unit = try!(
            self.unit_factory.create(id, type_id, input_channels, output_channels)
        );
        self.units.insert(id, unit);
        Ok(())
    }
}
