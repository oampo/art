use types::{ArtResult, UnitMap};
use errors::{UnitNotFoundError, UnownedUnitError, ParameterNotFoundError};
use graph::Graph;

#[derive(Copy)]
pub struct ParameterInstruction;

impl ParameterInstruction {
    pub fn link(unit_id: u32, owner: u32, units: &UnitMap, graph: &mut Graph)
            -> ArtResult<()> {
        let unit = try!(
            units.get(&unit_id).ok_or(
                UnitNotFoundError::new(unit_id)
            )
        );

        let to = try!(unit.owner.ok_or(UnownedUnitError::new(unit_id)));
        graph.add_edge(owner, to);

        Ok(())
    }

    pub fn run(unit_id: u32, id: u32, units: &mut UnitMap)
            -> ArtResult<()> {
        let mut unit = try!(
            units.get_mut(&unit_id).ok_or(
                UnitNotFoundError::new(unit_id)
            )
        );

        let mut parameter = try!(
            unit.data.get_parameters().get(id as usize).ok_or(
                ParameterNotFoundError::new(unit_id, id)
            )
        );

        // TODO: Make me work!

        Ok(())
    }
}