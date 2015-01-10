use std::vec::Vec;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use expression::Expression;
use unit::Unit;
use errors::ArtError;

pub type ByteCode = Vec<u8>;
pub type ByteCodeReceiver = Receiver<ByteCode>;

pub type UnitConstructor = fn(u32, u32) -> Box<Unit + 'static>;
pub type UnitId = u32;
pub type UnitTypeId = u32;
pub type UnitMap = HashMap<UnitId, Box<Unit + 'static>>;

pub type ExpressionId = u32;
pub type ExpressionMap = HashMap<ExpressionId, Expression>;

pub type ArtResult<T> = Result<T, ArtError>;
