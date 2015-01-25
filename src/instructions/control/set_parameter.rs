use types::ArtResult;
use errors::ArtError;

use vm_inner::VMInner;

pub trait SetParameter {
    fn set_parameter(&mut self, unit_id: u32, parameter_id: u32, value: f32)
            -> ArtResult<()>;
}

impl SetParameter for VMInner {
    fn set_parameter(&mut self, unit_id: u32, parameter_id: u32, value: f32)
            -> ArtResult<()> {
        debug!("Setting parameter: unit_id={}, parameter_id={}, value={}",
               unit_id, parameter_id, value);
        let mut unit = try!(
            self.units.get_mut(&unit_id).ok_or(
                ArtError::UnitNotFound {
                    unit_id: unit_id
                }
            )
        );

        let parameter = try!(
            unit.data.get_parameters().get_mut(parameter_id as usize).ok_or(
                ArtError::ParameterNotFound {
                    unit_id: unit_id,
                    parameter_id: parameter_id
                }
            )
        );

        parameter.value = value;

        Ok(())
    }
}
