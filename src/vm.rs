use std::int;
use std::uint;
use std::collections::HashMap;
use std::collections::RingBuf;

use portaudio::stream::{StreamCallbackResult, StreamTimeInfo,
                        StreamCallbackFlags};

use types::ArtResult;
use errors::{InvalidByteCodeError, ExpressionNotFoundError, PortAudioError};
use device::Device;
use unit_factory::UnitFactory;
use tickable::TickableBox;
use expression::Expression;
use opcode::Opcode;
use util::get_int_env_aliased;

pub type ByteCode = Vec<u32>;
pub type ByteCodeReceiver = Receiver<Vec<u32>>;
pub type UnitMap = HashMap<u32, TickableBox>;
type ExpressionMap = HashMap<u32, Expression>;

pub struct VM<'a> {
    input_channel: ByteCodeReceiver,
    units:UnitMap,
    expressions:ExpressionMap,
    unit_factory:UnitFactory
}

impl<'a> VM<'a> {
    pub fn new(input_channel: ByteCodeReceiver) -> VM<'a> {
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
        let (exit_signal_tx, exit_signal_rx) :
                (SyncSender<()>, Receiver<()>) = sync_channel(1);
        let stream = try!(
            device.open(|input_block: &[f32], output_block: &mut[f32],
                         time_info: StreamTimeInfo,
                         flags: StreamCallbackFlags| {
                self.tick();
                StreamCallbackResult::Continue
            })
        );

        try!(device.start());
        exit_signal_rx.recv();
        Ok(())
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
                    let result = self.process_byte_code(byte_code);
                    result.unwrap_or_else(|error| error!("{}", error));
                },
                Err(error) => { return; }
            }
            return;
        }
    }

    fn process_byte_code(&mut self, byte_code: Vec<u32>) -> ArtResult<()> {
        match byte_code.as_slice() {
            [opcode, instructions..]
                    if opcode == Opcode::Expression as u32 => {
                self.add_expression(instructions)
            },
            [opcode, id, type_id, input_channels, output_channels]
                    if opcode == Opcode::CreateUnit as u32 => {
                self.create_unit(id, type_id, input_channels,
                                 output_channels)
            },
            _ => {
                Err(InvalidByteCodeError::new())
            }
        }
    }

    fn add_expression(&mut self, byte_code: &[u32]) -> ArtResult<()> {
        let id = byte_code[0];
        let expression = try!(Expression::new(byte_code));
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


