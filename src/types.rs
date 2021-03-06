use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use errors::ArtError;

use expression::Expression;
use unit::Unit;
use parameter::Parameter;
use leap::Leap;

#[derive(Copy)]
pub struct ByteCode {
    pub data: [u8; 1024],
    pub size: usize
}
pub type ByteCodeReceiver = Receiver<ByteCode>;

pub type ExpressionId = u32;
pub type UnitId = u32;
pub type ParameterId = u32;
pub type UnitTypeId = u32;

pub type ExpressionMap = HashMap<ExpressionId, Expression>;
pub type UnitMap = HashMap<(ExpressionId, UnitId), Unit>;
pub type ParameterMap = HashMap<(ExpressionId, UnitId, ParameterId), Parameter>;
pub type BusMap = HashMap<u32, usize>;

pub type UnitConstructor = fn((u32, u32), u32, u32, &mut Leap<f32>) -> Unit;

pub type ArtResult<T> = Result<T, ArtError>;

#[derive(Copy, Clone, RustcEncodable, Debug, FromPrimitive, PartialEq)]
pub enum Rate {
    Audio,
    Control
}

#[derive(Copy, Clone)]
pub struct StackRecord {
    pub channels: u32,
    pub rate: Rate
}
