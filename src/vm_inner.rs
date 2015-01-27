use std::mem;
use std::collections::HashMap;

use portaudio::stream::{StreamCallbackResult, StreamTimeInfo,
                        StreamCallbackFlags};

use types::{ByteCodeReceiver, UnitMap, ExpressionMap, ParameterMap};
use sizes::BLOCK_SIZE;
use unit_factory::UnitFactory;
use channel_stack::ChannelStack;
use graph::Graph;

use phases::process::Process;
use phases::link::Link;
use phases::sort::Sort;
use phases::run::Run;
use phases::clean::Clean;

pub struct VMInner {
    pub input_channel: ByteCodeReceiver,
    pub unit_factory: UnitFactory,
    pub expressions: ExpressionMap,
    pub units: UnitMap,
    pub parameters: ParameterMap,
    pub graph: Graph,
    pub expression_ids: Vec<u32>,
    pub stack_data: Vec<f32>,
    pub bus_data: Vec<f32>
}

impl VMInner {
    pub fn new(input_channel: ByteCodeReceiver) -> VMInner {
        // TODO: Make sizes options
        let mut stack_data = Vec::with_capacity(32 * BLOCK_SIZE);
        stack_data.resize(32 * BLOCK_SIZE, 0f32);

        let mut bus_data = Vec::with_capacity(32 * BLOCK_SIZE);
        bus_data.resize(32 * BLOCK_SIZE, 0f32);

        VMInner {
            input_channel: input_channel,
            unit_factory: UnitFactory::new(),
            expressions: HashMap::new(),
            units: HashMap::new(),
            parameters: HashMap::new(),
            graph: Graph::new(16),
            expression_ids: Vec::with_capacity(32),
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
                                           BLOCK_SIZE);
        self.process();
        self.link(&mut busses);
        self.sort();
        self.run(&mut busses, adc_block, dac_block);
        self.clean();
    }

}


impl<'a, 'b> FnMut<
    (&'a [f32], &'b mut [f32], StreamTimeInfo, StreamCallbackFlags),
    (StreamCallbackResult)
> for VMInner {
    extern "rust-call" fn call_mut(&mut self, args: (&[f32], &mut [f32],
                                                     StreamTimeInfo,
                                                     StreamCallbackFlags))
            -> StreamCallbackResult {
        let (adc_block, dac_block, _, _) = args;
        self.tick(adc_block, dac_block)
    }
}

