use types::ArtResult;
use errors::ArtError;
use vm_inner::VMInner;

pub trait Parameter {
    fn link_parameter(&mut self, unit_id: u32, parameter_id: u32, from_id: u32)
            -> ArtResult<()>;
    fn tick_parameter(&mut self, unit_id: u32, parameter_id: u32)
            -> ArtResult<()>;
}

impl Parameter for VMInner {
    fn link_parameter(&mut self, unit_id: u32, parameter_id: u32, from_id: u32)
            -> ArtResult<()> {
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

        let to_id = try!(unit.owner.ok_or(
            ArtError::UnownedUnit {
                unit_id: unit_id
            }
        ));

        let bus_id = self.busses.reserve(1us);
        parameter.bus = Some(bus_id);

        self.graph.add_edge(from_id, to_id);

        Ok(())
    }

    fn tick_parameter(&mut self, unit_id: u32, parameter_id: u32)
            -> ArtResult<()> {
        let mut unit = try!(
            self.units.get_mut(&unit_id).ok_or(
                ArtError::UnitNotFound {
                    unit_id: unit_id
                }
            )
        );

        let parameter = try!(
            unit.data.get_parameters().get(parameter_id as usize).ok_or(
                ArtError::ParameterNotFound {
                    unit_id: unit_id,
                    parameter_id: parameter_id
                }
            )
        );

        let bus_id = try!(
            parameter.bus.ok_or(
                ArtError::UnlinkedParameter {
                    unit_id: unit_id,
                    parameter_id: parameter_id
                }
            )
        );


        let end = self.channel_stack.position;
        try!(self.channel_stack.pop_expect(1));
        let start = self.channel_stack.position;

        self.busses.set(bus_id, &self.channel_stack.data[start..end]);
        Ok(())
    }
}
