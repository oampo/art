use std::collections::HashMap;
use std::io::BufReader;

use portaudio::stream::{StreamCallbackResult, StreamTimeInfo,
                        StreamCallbackFlags};

use types::{ArtResult, ByteCodeReceiver, UnitMap, ExpressionMap};
use errors::{InvalidByteCodeError, ExpressionNotFoundError,
             UnimplementedOpcodeError};
use unit_factory::UnitFactory;
use expression::Expression;
use opcode::Opcode;
use opcode_reader::OpcodeReader;
use channel_stack::ChannelStack;
use graph::Graph;

pub struct VMInner {
    input_channel: ByteCodeReceiver,
    units: UnitMap,
    expressions: ExpressionMap,
    unit_factory: UnitFactory,
    channel_stack: ChannelStack,
    graph: Graph
}

impl VMInner {
    pub fn new(input_channel: ByteCodeReceiver) -> VMInner {
        VMInner {
            input_channel: input_channel,
            units: HashMap::new(),
            expressions: HashMap::new(),
            unit_factory: UnitFactory::new(),
            // TODO: Make num channels into option
            channel_stack: ChannelStack::new(16),
            graph: Graph::new(16, 16)
        }
    }

    fn tick(&mut self, adc_block: &[f32], dac_block: &mut [f32])
            -> StreamCallbackResult {
        self.process_queue();
        self.execute_expressions(adc_block, dac_block);
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
        let expression = Expression::new(opcodes);
        self.expressions.insert(id, expression);
        Ok(())
    }

    fn remove_expression(&mut self, id: u32) -> ArtResult<(Expression)> {
        let expression = self.expressions.remove(&id);
        expression.ok_or(ExpressionNotFoundError::new(id))
    }

    fn execute_expressions(&mut self, adc_block: &[f32],
                                      dac_block: &mut [f32]) {
        // TODO: Cache result, and dirty check edges
        // Reset the incoming edges
        for (_, expression) in self.expressions.iter_mut() {
            expression.incoming_edges = 0;
        }

        self.graph.topological_sort(&mut self.expressions);



/*
        let units = &mut self.units;
        let channel_stack = &mut self.channel_stack;
        for (_, expression) in self.expressions.iter_mut() {
            let result = expression.execute(channel_stack, units, adc_block, dac_block);
            result.unwrap_or_else(|error| error!("{:?}", error));
        }
        */
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

