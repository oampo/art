use std::collections::HashMap;

use portaudio::stream::{StreamCallbackResult, StreamTimeInfo,
                        StreamCallbackFlags};

use types::{ByteCodeReceiver, UnitMap, ExpressionMap};
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
    pub units: UnitMap,
    pub expressions: ExpressionMap,
    pub unit_factory: UnitFactory,
    pub channel_stack: ChannelStack,
    pub expression_ids: Vec<u32>,
    pub graph: Graph
}

impl VMInner {
    pub fn new(input_channel: ByteCodeReceiver) -> VMInner {
        // TODO: Make sizes options
        VMInner {
            input_channel: input_channel,
            units: HashMap::new(),
            expressions: HashMap::new(),
            unit_factory: UnitFactory::new(),
            channel_stack: ChannelStack::new(16),
            expression_ids: Vec::with_capacity(32),
            graph: Graph::new(16)
        }
    }

    fn tick(&mut self, adc_block: &[f32], dac_block: &mut [f32])
            -> StreamCallbackResult {
        self.process();
        self.link();
        self.sort();
        self.run(adc_block, dac_block);
        self.clean();
        StreamCallbackResult::Continue
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

