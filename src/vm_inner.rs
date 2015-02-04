use std::mem;
use std::old_io;
use std::old_io::fs::{mkdir_recursive, File, PathExtensions};
use std::collections::HashMap;

use rustc_serialize::{Encoder, Encodable, json};

use portaudio::stream::{StreamCallbackResult, StreamTimeInfo,
                        StreamCallbackFlags};

use util;
use types::{ByteCodeReceiver, UnitMap, ExpressionMap, ParameterMap, ArtResult};
use options::Options;
use unit_factory::UnitFactory;
use channel_stack::ChannelStack;
use graph::Graph;
use expression_list::ExpressionList;
use constants::Constants;

use phases::process::Process;
use phases::link::Link;
use phases::sort::Sort;
use phases::run::Run;
use phases::clean::Clean;

pub struct VmInner {
    pub input_channel: ByteCodeReceiver,
    pub constants: Constants,
    pub unit_factory: UnitFactory,
    pub expressions: ExpressionMap,
    pub expression_list: ExpressionList,
    pub units: UnitMap,
    pub parameters: ParameterMap,
    pub graph: Graph,
    pub expression_ids: Vec<u32>,
    pub stack_data: Vec<f32>,
    pub bus_data: Vec<f32>,
}

impl VmInner {
    pub fn new(options: &Options, input_channel: ByteCodeReceiver)
            -> VmInner {
        let stack_data_size = (
            options.num_stack_channels * options.block_size
        ) as usize;
        let mut stack_data = Vec::with_capacity(stack_data_size);
        stack_data.resize(stack_data_size, 0f32);

        let bus_data_size = (
            options.num_bus_channels * options.block_size
        ) as usize;
        let mut bus_data = Vec::with_capacity(bus_data_size);
        bus_data.resize(bus_data_size, 0f32);

        VmInner {
            input_channel: input_channel,
            constants: Constants {
                input_channels: options.input_channels,
                output_channels: options.output_channels,
                block_size: options.block_size as usize,
                block_size_inverse: 1f32 / options.block_size as f32,
                audio_rate: options.sample_rate as f32,
                audio_rate_inverse: 1f32 / options.sample_rate as f32
            },
            unit_factory: UnitFactory::new(),
            expression_list: ExpressionList::with_capacity(
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
            graph: Graph::with_capacity(options.max_edges),
            expression_ids: Vec::with_capacity(
                options.max_expressions as usize
            ),
            stack_data: stack_data,
            bus_data: bus_data
        }
    }

    fn tick(&mut self, adc_block: &[f32], dac_block: &mut [f32])
            -> StreamCallbackResult {
        let mut bus_data = Vec::with_capacity(0);
        mem::swap(&mut self.bus_data, &mut bus_data);
        self.tick_inner(&mut bus_data, adc_block, dac_block);
        mem::swap(&mut self.bus_data, &mut bus_data);
        StreamCallbackResult::Continue
    }

    fn tick_inner(&mut self, bus_data: &mut Vec<f32>,
                 adc_block: &[f32], dac_block: &mut [f32]) {
        let mut busses = ChannelStack::new(bus_data.as_mut_slice(),
                                           self.constants.block_size);
        self.process();
        self.link(&mut busses);
        self.sort();
        self.run(&mut busses, adc_block, dac_block);
        self.clean();
    }

    pub fn write_info_file(&self) -> ArtResult<()> {
        let mut path = util::user_data_dir().unwrap();
        if !path.exists() {
            try!(mkdir_recursive(&path, old_io::USER_DIR));
        }

        path.push("art_info.json");

        let mut file = File::create(&path);
        try!(
            file.write_all(
                json::encode(self).unwrap().into_bytes().as_slice()
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



