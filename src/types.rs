use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use errors::ArtError;

use expression::Expression;
use unit::Unit;
use parameter::Parameter;

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

pub type UnitConstructor = fn((u32, u32), u32, u32) -> Unit;

pub type ArtResult<T> = Result<T, ArtError>;
