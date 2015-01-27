use types::ArtResult;
use errors::ArtError;

use vm_inner::VMInner;

pub trait SetParameter {
    fn set_parameter(&mut self, id: (u32, u32, u32), value: f32)
            -> ArtResult<()>;
}

impl SetParameter for VMInner {
    fn set_parameter(&mut self, id: (u32, u32, u32), value: f32)
            -> ArtResult<()> {
        let (uid, eid, pid) = id;
        debug!("Setting parameter: expression_id={}, unit_id={},
                parameter_id={}, value={}", eid, uid, pid, value);

        let parameter = try!(
            self.parameters.get_mut(&id).ok_or(
                ArtError::ParameterNotFound {
                    expression_id: eid,
                    unit_id: uid,
                    parameter_id: pid
                }
            )
        );

        parameter.value = value;

        Ok(())
    }
}
