use std::str::SendStr;

use portaudio::pa::PaError;

#[deriving(Show)]
pub struct ArtError {
    kind: ArtErrorKind,
    message: SendStr,
    detail: SendStr
}

#[deriving(Show)]
pub enum ArtErrorKind {
    UndefinedUnit { type_id: u32 },
    UnitNotFound { unit_id: u32 },
    ExpressionNotFound { expression_id: u32 },
    InvalidByteCode,
    InvalidStack,
    PortAudio { error: PaError }
}

impl ArtError {
    pub fn new<T: IntoCow<'static, String, str>>(kind: ArtErrorKind, msg: T,
                                           detail: T)
            -> ArtError {
        ArtError {
            kind: kind,
            message: msg.into_cow(),
            detail: "".into_cow()
        }
    }
}

pub struct UndefinedUnitError;

impl UndefinedUnitError {
    pub fn new(type_id: u32) -> ArtError {
        ArtError::new(ArtErrorKind::UndefinedUnit { type_id: type_id },
                      "Unit is undefined", "")
    }
}

pub struct UnitNotFoundError;

impl UnitNotFoundError {
    pub fn new(unit_id: u32) -> ArtError {
        ArtError::new(ArtErrorKind::UnitNotFound { unit_id: unit_id },
                      "Unit not found", "")
    }
}

pub struct InvalidByteCodeError;

impl InvalidByteCodeError {
    pub fn new() -> ArtError {
        ArtError::new(ArtErrorKind::InvalidByteCode,
                      "Invalid bytecode", "")
    }
}

pub struct ExpressionNotFoundError;

impl ExpressionNotFoundError {
    pub fn new(expression_id: u32) -> ArtError {
        ArtError::new(ArtErrorKind::ExpressionNotFound {
                          expression_id: expression_id
                      }, "Expression not found", "")
    }
}

pub struct InvalidStackError;

impl InvalidStackError {
    pub fn new() -> ArtError {
        ArtError::new(ArtErrorKind::InvalidStack, "Invalid stack", "")
    }
}

pub struct PortAudioError;

impl PortAudioError {
    pub fn new(error: PaError) -> ArtError {
        ArtError::new(ArtErrorKind::PortAudio { error : error },
                      "PortAudio error", "")
    }
}
