use std::collections::HashMap;
use std::io::{BufReader, IoError};

use portaudio::stream::{StreamCallbackResult, StreamTimeInfo,
                        StreamCallbackFlags};

use types::ArtResult;
use errors::{InvalidByteCodeError, ExpressionNotFoundError};
use device::Device;
use unit_factory::UnitFactory;
use tickable::TickableBox;
use expression::Expression;
use opcode::{Opcode, OpcodeType};
use opcode_reader::OpcodeReader;
use util::get_int_env_aliased;

pub type ByteCodeReceiver = Receiver<Vec<u8>>;
pub type UnitMap = HashMap<u32, TickableBox>;
pub type ExpressionMap = HashMap<u32, Expression>;

pub struct VM {
    input_channel: ByteCodeReceiver,
    units:UnitMap,
    expressions:ExpressionMap,
    unit_factory:UnitFactory
}

impl VM {
    pub fn new(input_channel: ByteCodeReceiver) -> VM {
        VM {
            input_channel: input_channel,
            units: HashMap::new(),
            expressions: HashMap::new(),
            unit_factory: UnitFactory::new()
        }
    }

    pub fn run(&mut self) -> ArtResult<()> {
        let input_device = get_int_env_aliased("ART_INPUT_DEVICE",
                                               "ART_DEVICE").unwrap_or(-1);
        let output_device = get_int_env_aliased("ART_OUTPUT_DEVICE",
                                                "ART_DEVICE").unwrap_or(-1);
        let input_channels : uint = get_int_env_aliased(
            "ART_INPUT_CHANNELS", "ART_CHANNELS"
        ).unwrap_or(0) as uint;
        let output_channels : uint = get_int_env_aliased(
            "ART_OUTPUT_CHANNELS", "ART_CHANNELS"
        ).unwrap_or(1) as uint;

        let mut device = Device::new(
            input_device, output_device, input_channels, output_channels,
        );

        let (exit_channel_sender, exit_channel_receiver):
                (SyncSender<()>, Receiver<()>) = sync_channel(1);

        try!(
            device.open(|input_block: &[f32], output_block: &mut[f32],
                         time_info: StreamTimeInfo,
                         flags: StreamCallbackFlags| {
                self.tick()
            })
        );

        try!(device.start());

        exit_channel_receiver.recv();
        Ok(())
    }

    fn tick(&mut self) -> StreamCallbackResult {
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


