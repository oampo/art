use std::mem;
use std::old_io::{self, BufReader, IoError, IoErrorKind};
use std::old_io::fs::{mkdir_recursive, File, PathExtensions};
use std::collections::HashMap;

use rustc_serialize::{Encoder, Encodable, json};

use portaudio::stream::{StreamCallbackResult, StreamTimeInfo,
                        StreamCallbackFlags};

use util;
use types::{ByteCodeReceiver, UnitMap, ExpressionMap, ParameterMap, BusMap,
            StackRecord, ArtResult};
use unit::TickAdjuncts;
use errors::ArtError;
use options::Options;
use opcode::{ControlOpcode, DspOpcode};
use opcode_reader::OpcodeReader;
use unit_factory::UnitFactory;
use channel_stack::ChannelStack;
use graph::Graph;
use expression::{Expression, ExpressionState};
use leap::Leap;
use expression_store::ExpressionStore;
use validator::ExpressionValidator;
use constants::Constants;

pub struct VmInner {
    pub input_channel: ByteCodeReceiver,
    pub constants: Constants,
    pub unit_factory: UnitFactory,
    pub expressions: ExpressionMap,
    pub expression_store: Leap<DspOpcode>,
    pub units: UnitMap,
    pub parameters: ParameterMap,
    pub bus_map: BusMap,
    pub graph: Graph,
    pub expression_ids: Vec<u32>,
    pub stack_data: Vec<f32>,
    pub bus_data: Vec<f32>,
    pub stack_record: Vec<StackRecord>
}

impl VmInner {
    pub fn new(options: &Options, input_channel: ByteCodeReceiver)
            -> VmInner {
        let mut stack_data = Vec::with_capacity(options.stack_size);
        stack_data.resize(options.stack_size, 0f32);

        let mut bus_data = Vec::with_capacity(options.bus_stack_size);
        bus_data.resize(options.bus_stack_size, 0f32);

        VmInner {
            input_channel: input_channel,
            constants: Constants {
                input_channels: options.input_channels,
                output_channels: options.output_channels,
                block_size: options.block_size as usize,
                block_size_inverse: 1f32 / options.block_size as f32,
                audio_rate: options.sample_rate as f32,
                audio_rate_inverse: 1f32 / options.sample_rate as f32,
                control_rate: options.sample_rate as f32 /
                              options.block_size as f32,
                control_rate_inverse: options.block_size as f32 /
                                      options.sample_rate as f32
            },
            unit_factory: UnitFactory::new(),
            expression_store: Leap::with_capacity(
                options.max_opcodes as usize
            ),
            expressions: HashMap::with_capacity(
                options.max_expressions as usize
            ),
            units: HashMap::with_capacity(
                options.max_units as usize
            ),
            parameters: HashMap::with_capacity(
                options.max_parameters as usize
            ),
            bus_map: HashMap::with_capacity(
                options.max_bus_depth as usize
            ),
            graph: Graph::with_capacity(options.max_edges),
            expression_ids: Vec::with_capacity(
                options.max_expressions as usize
            ),
            stack_data: stack_data,
            bus_data: bus_data,
            stack_record: Vec::with_capacity(
                options.max_stack_depth as usize
            )
        }
    }

    fn tick(&mut self, adc_block: &[f32], dac_block: &mut [f32])
            -> StreamCallbackResult {
        self.read();
        for id in self.expressions.keys() {
            self.expression_ids.push(*id);
        }
        // TODO: We should be able to do a dirty check, and only regenerate
        // the graph when something has changed
        self.graph.topological_sort(&mut self.expressions,
                                    &mut self.expression_ids);
        self.run(adc_block, dac_block);
        self.clean();
        StreamCallbackResult::Continue
    }

    /* Phases */
    pub fn read(&mut self) {
        let result = self.input_channel.try_recv();
        if let Ok(byte_code) = result {
            let result = self.process(&byte_code.data[..byte_code.size]);
            result.unwrap_or_else(|error| error!("{}", error));
        }
    }

    fn process(&mut self, byte_code: &[u8]) -> ArtResult<()> {
        let mut reader = BufReader::new(byte_code);
        while !reader.eof() {
            let opcode = try!(reader.read_control_opcode());
            try!(self.process_opcode(opcode, &mut reader));
        }
        Ok(())
    }

    fn process_opcode(&mut self, opcode: ControlOpcode,
                      reader: &mut BufReader) -> ArtResult<()> {
        match opcode {
            ControlOpcode::AddExpression { expression_id, num_opcodes } => {
                let num_opcodes = num_opcodes as usize;
                let start = try!(
                    self.expression_store.push_from_reader(num_opcodes,
                                                           reader)
                );
                self.add_expression(expression_id, start, num_opcodes)
            },
            ControlOpcode::RemoveExpression { expression_id } => {
                self.remove_expression(expression_id)
            },
            ControlOpcode::SetParameter { expression_id, unit_id,
                                          parameter_id, value } => {
                self.set_parameter((expression_id, unit_id, parameter_id),
                                   value)
            },
            ControlOpcode::AddEdge { from, to } => {
                self.graph.add_edge(from, to);
                Ok(())
            }
        }
    }

    pub fn prepare_io_busses(busses: &mut ChannelStack,
                             adc_block: &[f32], dac_block: &mut [f32])
            -> ArtResult<(usize, usize)> {
        let adc_index = try!(busses.push(adc_block.len()));
        let dac_index = try!(busses.push(dac_block.len()));
        busses.write(adc_index, adc_block);
        busses.zero(dac_index, dac_block.len());
        Ok((adc_index, dac_index))
    }

    pub fn run(&mut self, adc_block: &[f32], dac_block: &mut [f32]) {
        let mut busses = ChannelStack::new(&mut self.bus_data);
        // TODO: Nicer error message when there are not enough busses for the
        // ADC/DAC
        let (adc_index, dac_index) = VmInner::prepare_io_busses(
            &mut busses, adc_block, dac_block
        ).unwrap();
        self.bus_map.insert(0, adc_index);
        self.bus_map.insert(1, dac_index);

        let expression_ids = mem::replace(&mut self.expression_ids,
                                          Vec::with_capacity(0));

        let mut adjuncts = TickAdjuncts {
            busses: &mut busses,
            bus_map: &mut self.bus_map,
            parameters: &mut self.parameters
        };

        for id in expression_ids.iter() {
            debug_assert!(self.expressions.contains_key(id));
            let expression = self.expressions.get_mut(id).unwrap();
            let mut stack = ChannelStack::new(&mut self.stack_data);
            let result = expression.tick(
                &self.expression_store, &mut stack, &mut self.units,
                &mut adjuncts, &self.constants
            );

            if result.is_err() {
                expression.state = ExpressionState::Free;
                debug!("Expression tick failed: reason={}",
                       result.err().unwrap());
            }
        }
        self.expression_ids = expression_ids;
        adjuncts.busses.read(dac_index, dac_block);
    }

    pub fn clean(&mut self) {
        // Remove failed and freed expressions
        self.expression_ids.clear();
        for (&id, expression) in self.expressions.iter() {
            if expression.state == ExpressionState::Free {
                self.expression_ids.push(id);
            }
        };

        for id in self.expression_ids.iter() {
            debug_assert!(self.expressions.contains_key(id));
            let expression = self.expressions.remove(id).unwrap();
            expression.free_units(&self.expression_store, &mut self.units,
                                  &mut self.parameters);
        }

        // Remove freed nodes from the edge list
        self.graph.clear(&self.expression_ids);


        // Reset things which are rebuilt on each tick
        self.bus_map.clear();

        for (_, expression) in self.expressions.iter_mut() {
            expression.incoming_edges = 0;
        }
        self.expression_ids.clear();
    }

    /* Control instructions */
    pub fn add_expression(&mut self, id: u32, index: usize, num_opcodes: usize)
            -> ArtResult<()> {
        debug!("Adding expression: id={:?}, index={:?}", id, index);
        let result = ExpressionValidator::validate(
            index, num_opcodes, &self.expression_store, &mut self.stack_record,
            &self.unit_factory, &self.expressions, &self.units,
            &self.parameters
        );

        if result.is_err() {
            let _ = self.expression_store.free(index, num_opcodes);
            return result;
        }

        let expression = Expression::new(id, index, num_opcodes );

        let _ = expression.construct_units(
            &self.expression_store, &mut self.unit_factory, &mut self.units,
            &mut self.parameters
        );

        self.expressions.insert(id, expression);
        Ok(())
    }

    pub fn remove_expression(&mut self, expression_id: u32) -> ArtResult<()> {
        let expression = try!(
            self.expressions.get_mut(&expression_id).ok_or(
                ArtError::ExpressionNotFound {
                    expression_id: expression_id
                }
            )
        );

        expression.state = ExpressionState::Free;
        Ok(())
    }

    pub fn set_parameter(&mut self, id: (u32, u32, u32), value: f32)
            -> ArtResult<()> {
        let (eid, uid, pid) = id;
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

    pub fn write_info_file(&self) -> ArtResult<()> {
        let json = try!(json::encode(self));

        let mut path = try!(util::user_data_dir().ok_or(
            IoError {
                kind: IoErrorKind::OtherIoError,
                desc: "Could not determine user data directory",
                detail: None
            }
        ));
        if !path.exists() {
            try!(mkdir_recursive(&path, old_io::USER_DIR));
        }

        path.push("art_info.json");

        let mut file = File::create(&path);
        try!(
            file.write_all(
                json.into_bytes().as_slice()
            )
        );
        Ok(())
    }
}


impl<'a, 'b> FnMut<
    (&'a [f32], &'b mut [f32], StreamTimeInfo, StreamCallbackFlags)
> for VmInner {
    type Output = StreamCallbackResult;
    extern "rust-call" fn call_mut(&mut self, args: (&[f32], &mut [f32],
                                                     StreamTimeInfo,
                                                     StreamCallbackFlags))
            -> StreamCallbackResult {
        let (adc_block, dac_block, _, _) = args;
        self.tick(adc_block, dac_block)
    }
}

impl Encodable for VmInner {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_struct("VmInner", 5, |encoder| {
            try!(
                encoder.emit_struct_field("input_channels", 0, |encoder|
                    self.constants.input_channels.encode(encoder)
                )
            );
            try!(
                encoder.emit_struct_field("output_channels", 1, |encoder|
                    self.constants.output_channels.encode(encoder)
                )
            );
            try!(
                encoder.emit_struct_field("sample_rate", 2, |encoder|
                    self.constants.audio_rate.encode(encoder)
                )
            );
            try!(
                encoder.emit_struct_field("block_size", 3, |encoder|
                    self.constants.block_size.encode(encoder)
                )
            );
            try!(
                encoder.emit_struct_field("units", 4, |encoder|
                    self.unit_factory.units.encode(encoder)
                )
            );
            Ok(())
        })
    }
}



