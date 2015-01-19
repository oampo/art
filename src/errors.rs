use std::string::CowString;
use std::borrow::IntoCow;

use portaudio::pa::PaError;

use opcode::Opcode;

#[derive(Show)]
pub struct ArtError {
    kind: ArtErrorKind,
    message: CowString<'static>,
    detail: CowString<'static>
}

#[derive(Show)]
pub enum ArtErrorKind {
    UnimplementedOpcode { opcode: Opcode },
    UndefinedUnit { type_id: u32 },
    UnitNotFound { unit_id: u32 },
    ParameterNotFound { unit_id: u32, id: u32 },
    ExpressionNotFound { expression_id: u32 },
    InvalidChannelCount,
    InvalidByteCode,
    StackFull,
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
            detail: detail.into_cow()
        }
    }
}

#[derive(Copy)]
pub struct UnimplementedOpcodeError;

impl UnimplementedOpcodeError {
    pub fn new(opcode:Opcode) -> ArtError {
        ArtError::new(ArtErrorKind::UnimplementedOpcode { opcode: opcode },
                      "Opcode is unimplemented", "")
    }
}

#[derive(Copy)]
pub struct UndefinedUnitError;

impl UndefinedUnitError {
    pub fn new(type_id: u32) -> ArtError {
        ArtError::new(ArtErrorKind::UndefinedUnit { type_id: type_id },
                      "Unit is undefined", "")
    }
}

#[derive(Copy)]
pub struct UnitNotFoundError;

impl UnitNotFoundError {
    pub fn new(unit_id: u32) -> ArtError {
        ArtError::new(ArtErrorKind::UnitNotFound { unit_id: unit_id },
                      "Unit not found", "")
    }
}

#[derive(Copy)]
pub struct ParameterNotFoundError;

impl ParameterNotFoundError {
    pub fn new(unit_id: u32, id: u32) -> ArtError {
        ArtError::new(ArtErrorKind::ParameterNotFound {
                        unit_id: unit_id,
                        id: id
                      }, "Parameter not found", "")
    }
}

#[derive(Copy)]
pub struct InvalidByteCodeError;

impl InvalidByteCodeError {
    pub fn new() -> ArtError {
        ArtError::new(ArtErrorKind::InvalidByteCode,
                      "Invalid bytecode", "")
    }
}

#[derive(Copy)]
pub struct ExpressionNotFoundError;

impl ExpressionNotFoundError {
    pub fn new(expression_id: u32) -> ArtError {
        ArtError::new(ArtErrorKind::ExpressionNotFound {
                          expression_id: expression_id
                      }, "Expression not found", "")
    }
}


#[derive(Copy)]
pub struct InvalidChannelCountError;

impl InvalidChannelCountError {
    pub fn new() -> ArtError {
        ArtError::new(ArtErrorKind::InvalidChannelCount,
                      "Invalid channel count", "")
    }
}

#[derive(Copy)]
pub struct StackFullError;

impl StackFullError {
    pub fn new() -> ArtError {
        ArtError::new(ArtErrorKind::StackFull, "Full stack", "")
    }
}

#[derive(Copy)]
pub struct InvalidStackError;

impl InvalidStackError {
    pub fn new() -> ArtError {
        ArtError::new(ArtErrorKind::InvalidStack, "Invalid stack", "")
    }
}

#[derive(Copy)]
pub struct PortAudioError;

impl PortAudioError {
    pub fn new(error: PaError) -> ArtError {
        ArtError::new(ArtErrorKind::PortAudio { error : error },
                      "PortAudio error", "")
    }
}
