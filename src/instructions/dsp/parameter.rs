use types::ArtResult;
use errors::{UnitNotFoundError, UnownedUnitError, ParameterNotFoundError};
use vm_inner::VMInner;

pub trait Parameter {
    fn link_parameter(&mut self, unit_id: u32, id: u32, owner_id: u32)
            -> ArtResult<()>;
    fn tick_parameter(&mut self, unit_id: u32, id: u32) -> ArtResult<()>;
}

impl Parameter for VMInner {
    fn link_parameter(&mut self, unit_id: u32, id: u32, owner_id: u32)
            -> ArtResult<()> {
        let unit = try!(
            self.units.get(&unit_id).ok_or(
                UnitNotFoundError::new(unit_id)
            )
        );

        let to = try!(unit.owner.ok_or(UnownedUnitError::new(unit_id)));
        self.graph.add_edge(owner_id, to);

        Ok(())
    }

    fn tick_parameter(&mut self, unit_id: u32, id: u32) -> ArtResult<()> {
        let mut unit = try!(
            self.units.get_mut(&unit_id).ok_or(
                UnitNotFoundError::new(unit_id)
            )
        );

        let parameter = try!(
            unit.data.get_parameters().get(id as usize).ok_or(
                ParameterNotFoundError::new(unit_id, id)
            )
        );

        // TODO: Make me work!

        Ok(())
    }
}
