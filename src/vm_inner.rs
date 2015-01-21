use std::mem;
use std::collections::HashMap;
use std::io::BufReader;

use portaudio::stream::{StreamCallbackResult, StreamTimeInfo,
                        StreamCallbackFlags};

use types::{ArtResult, ByteCodeReceiver, UnitMap, ExpressionMap};
use errors::{InvalidByteCodeError, UnimplementedOpcodeError,
             ExpressionNotFoundError};
use unit_factory::UnitFactory;
use expression::Expression;
use opcode::{Opcode, ControlOpcode, DspOpcode};
use opcode_reader::OpcodeReader;
use channel_stack::ChannelStack;
use graph::{Graph, Node};

use instructions::control::create_unit::CreateUnit;
use instructions::control::add_expression::AddExpression;
use instructions::dsp::unit::Unit;
use instructions::dsp::dac::Dac;
use instructions::dsp::parameter::Parameter;

pub struct VMInner {
    input_channel: ByteCodeReceiver,
    pub units: UnitMap,
    pub expressions: ExpressionMap,
    pub unit_factory: UnitFactory,
    pub channel_stack: ChannelStack,
    expression_ids: Vec<u32>,
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
        self.process_queue();
//        self.link_expressions();
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
            reader.read_control_opcode().map_err(|_|
                InvalidByteCodeError::new()
            )
        );

        match opcode {
            ControlOpcode::AddExpression { id, opcodes } => {
                self.add_expression(id, opcodes)
            },

            ControlOpcode::CreateUnit { id, type_id, input_channels,
                                        output_channels } => {
                self.create_unit(id, type_id, input_channels, output_channels)
            },

            ControlOpcode::Unknown => Err(InvalidByteCodeError::new()),
            _ => Err(UnimplementedOpcodeError::new(Opcode::Control(opcode)))
        }
    }

    fn link_expressions(&mut self) {
        let mut expression_ids = Vec::<u32>::with_capacity(0);
        mem::swap(&mut self.expression_ids, &mut expression_ids);
        for &id in expression_ids.iter() {
            let result = self.link_expression(id);
            result.unwrap_or_else(|error| error!("{:?}", error));
        }
        mem::swap(&mut self.expression_ids, &mut expression_ids);
    }

    fn swap_expression(&mut self, id: u32,
                       expression: &mut Expression) -> ArtResult<()> {
        let expression_b = try!(
            self.expressions.get_mut(&id).ok_or(
                ExpressionNotFoundError::new(id)
            )
        );
        mem::swap(expression, expression_b);
        Ok(())
    }

    fn link_expression(&mut self, expression_id: u32) -> ArtResult<()> {
        let mut expression = Expression::new(Vec::with_capacity(0));
        try!(self.swap_expression(expression_id, &mut expression));

        for opcode in expression.opcodes.iter() {
            match opcode {
                &DspOpcode::Parameter { unit_id, id } => {
                    try!(
                        self.link_parameter(unit_id, id, expression_id)
                    )
                },
                _ => {}
            }
        }

        try!(self.swap_expression(expression_id, &mut expression));
        Ok(())
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

    fn run_expressions(&mut self, adc_block: &[f32], dac_block: &mut [f32]) {
        let mut expression_ids = Vec::<u32>::with_capacity(0);
        mem::swap(&mut self.expression_ids, &mut expression_ids);
        for id in expression_ids.iter() {
            let result = self.run_expression(*id, adc_block, dac_block);
            result.unwrap_or_else(|error| error!("{:?}", error));
        }
        mem::swap(&mut self.expression_ids, &mut expression_ids);
    }

    fn run_expression(&mut self, id: u32, adc_block: &[f32],
                      dac_block: &mut[f32]) -> ArtResult<()> {
        let mut expression = Expression::new(Vec::with_capacity(0));
        self.swap_expression(id, &mut expression);

        for opcode in expression.opcodes.iter() {
            match opcode {
                &DspOpcode::Unit { id } => {
                    try!(self.tick_unit(id))
                },
                &DspOpcode::Dac => {
                    try!(self.tick_dac(dac_block));
                },
                &DspOpcode::Parameter { unit_id, id } => {
                    try!(self.tick_parameter(unit_id, id));
                }
                _ => return Err(InvalidByteCodeError::new())
            }
        }

        self.swap_expression(id, &mut expression);
        Ok(())
    }

    fn cleanup(&mut self) {
        self.graph.clear();
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

