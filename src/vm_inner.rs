use std::collections::HashMap;
use std::io::BufReader;

use portaudio::stream::{StreamCallbackResult, StreamTimeInfo,
                        StreamCallbackFlags};

use types::{ArtResult, ByteCodeReceiver, UnitMap, ExpressionMap};
use errors::{InvalidByteCodeError, ExpressionNotFoundError};
use unit_factory::UnitFactory;
use tickable::Tickable;
use expression::Expression;
use opcode::Opcode;
use opcode_reader::OpcodeReader;

pub struct VMInner {
    input_channel: ByteCodeReceiver,
    units: UnitMap,
    expressions: ExpressionMap,
    unit_factory: UnitFactory
}

impl VMInner {
    pub fn new(input_channel: ByteCodeReceiver) -> VMInner {
        VMInner {
            input_channel: input_channel,
            units: HashMap::new(),
            expressions: HashMap::new(),
            unit_factory: UnitFactory::new()
        }
    }

    fn tick(&mut self) -> StreamCallbackResult {
        println!("Tick");
        self.process_queue();
        self.execute_expressions();
        StreamCallbackResult::Continue
    }

    fn process_queue(&mut self) {
        loop {
            let result = self.input_channel.try_recv();
            match result {
                Ok(byte_code) => {
                    let result = self.process_byte_code(byte_code.as_slice());
                    result.unwrap_or_else(|error| error!("{}", error));
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
            _ => unimplemented!()
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

    fn execute_expressions(&mut self) {
        let units = &mut self.units;
        for (_, expression) in self.expressions.iter_mut() {
            let result = expression.execute(units);
            result.unwrap_or_else(|error| error!("{}", error));
        }
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
    extern "rust-call" fn call_mut(&mut self, _: (&[f32], &mut [f32],
                                                  StreamTimeInfo,
                                                  StreamCallbackFlags))
            -> StreamCallbackResult {
        self.tick()
    }
}

