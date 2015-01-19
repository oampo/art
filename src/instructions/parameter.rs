use types::{ArtResult, UnitMap};
use errors::{UnitNotFoundError, ParameterNotFoundError};

#[derive(Copy)]
pub struct ParameterInstruction;

impl ParameterInstruction {
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
