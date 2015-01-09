use std::vec::Vec;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use tickable::Tickable;
use expression::Expression;
use errors::ArtError;

pub type ByteCode = Vec<u8>;
pub type ByteCodeReceiver = Receiver<ByteCode>;

pub type Unit = Box<Tickable + 'static>;
pub type UnitConstructor = fn(u32, u32) -> Unit;
pub type UnitId = u32;
pub type UnitTypeId = u32;
pub type UnitMap = HashMap<UnitId, Unit>;

pub type ExpressionId = u32;
pub type ExpressionMap = HashMap<ExpressionId, Expression>;

pub type ArtResult<T> = Result<T, ArtError>;
