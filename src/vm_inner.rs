use std::collections::HashMap;
use std::io::BufReader;

use portaudio::stream::{StreamCallbackResult, StreamTimeInfo,
                        StreamCallbackFlags};

use types::{ArtResult, ByteCodeReceiver, UnitMap, ExpressionMap};
use errors::{InvalidByteCodeError, UnimplementedOpcodeError};
use unit_factory::UnitFactory;
use expression::Expression;
use opcode::Opcode;
use opcode_reader::OpcodeReader;
use channel_stack::ChannelStack;
use graph::{Graph, Node};

pub struct VMInner {
    input_channel: ByteCodeReceiver,
    units: UnitMap,
    expressions: ExpressionMap,
    unit_factory: UnitFactory,
    channel_stack: ChannelStack,
    expression_ids: Vec<u32>,
    graph: Graph
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
        self.process_queue();
        self.link_expressions();
        self.sort_expressions();
        self.run_expressions(adc_block, dac_block);
        self.cleanup();
        StreamCallbackResult::Continue
    }

    fn process_queue(&mut self) {
        loop {
            let result = self.input_channel.try_recv();
            match result {
                Ok(byte_code) => {
                    let result = self.process_byte_code(byte_code.as_slice());
                    result.unwrap_or_else(|error| error!("{:?}", error));
                },
                Err(_) => { return; }
            }
            return;
        }
    }

    fn process_byte_code(&mut self, byte_code: &[u8]) -> ArtResult<()> {
        let mut reader = BufReader::new(byte_code);
        let opcode = try!(
            reader.read_opcode().map_err(|_| InvalidByteCodeError::new())
        );

        match opcode {
            Opcode::Expression { id, opcodes } => {
                self.add_expression(id, opcodes)
            },

            Opcode::CreateUnit { id, type_id, input_channels, output_channels } => {
                self.create_unit(id, type_id, input_channels, output_channels)
            },

            Opcode::Unknown => Err(InvalidByteCodeError::new()),
            _ => Err(UnimplementedOpcodeError::new(opcode))
        }
    }

    fn add_expression(&mut self, id: u32, opcodes: Vec<Opcode>)
            -> ArtResult<()> {
        let expression = Expression::new(id, opcodes);
        self.expressions.insert(id, expression);
        Ok(())
    }

    fn link_expressions(&mut self) {
        for (_, expression) in self.expressions.iter_mut() {
            expression.link(&self.units, &mut self.graph);
        }
    }

    fn sort_expressions(&mut self) {
        self.expression_ids.clear();

        for (id, expression) in self.expressions.iter_mut() {
            self.expression_ids.push(*id);
            expression.reset_edge_count();
        }

        self.graph.topological_sort(&mut self.expressions,
                                    self.expression_ids.as_mut_slice());
    }

    fn run_expressions(&mut self, adc_block: &[f32],
                                      dac_block: &mut [f32]) {
        for id in self.expression_ids.iter() {
            let expression = self.expressions.get_mut(id).unwrap();
            let result = expression.run(&mut self.channel_stack,
                                        &mut self.units,
                                        adc_block, dac_block);
            result.unwrap_or_else(|error| error!("{:?}", error));
        }
    }

    fn cleanup(&mut self) {
        self.graph.clear();
    }

    fn create_unit(&mut self, id: u32, type_id: u32, input_channels: u32,
                   output_channels: u32) -> ArtResult<()> {
        let unit = try!(
            self.unit_factory.create(type_id, input_channels, output_channels)
        );
        self.units.insert(id, unit);
        Ok(())
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

