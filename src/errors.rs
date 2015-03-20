use std::error::{Error, FromError};
use std::io;
use std::fmt;

use rustc_serialize::json::EncoderError;
use portaudio::pa::PaError;

use types::Rate;
use opcode::Opcode;

#[derive(Debug)]
pub enum ArtError {
    UnimplementedOpcode { opcode: Opcode },
    UndefinedUnit { type_id: u32 },
    IndexError,
    ExpressionNotFound { expression_id: u32 },
    UnitNotFound { expression_id: u32, unit_id: u32 },
    ParameterNotFound { expression_id: u32, unit_id: u32, parameter_id: u32 },
    ChannelMismatch { expected: u32, actual: u32 },
    RateMismatch { expected: Rate, actual: Rate },
    InvalidByteCode,
    IoError { error: io::Error },
    StackOverflow,
    StackUnderflow,
    BufferOverflow,
    InvalidStack,
    EncoderError { error:EncoderError },
    PortAudio { error: PaError }
}

impl ArtError {
    fn detail(&self) -> Option<String> {
        match *self {
            ArtError::UnimplementedOpcode { ref opcode } => {
                // FIXME: Add Display trait to opcode
                Some(format!("opcode={:?}", opcode))
            },
            ArtError::UndefinedUnit { type_id } => {
                Some(format!("type_id={}", type_id))
            },
            ArtError::ExpressionNotFound { expression_id } => {
                Some(format!("expression_id={}", expression_id))
            },
            ArtError::UnitNotFound { expression_id, unit_id } => {
                Some(format!("expression_id: {}, unit_id={}",
                             expression_id, unit_id))
            },
            ArtError::ParameterNotFound { expression_id, unit_id,
                                          parameter_id } => {
                Some(format!("expression_id={}, unit_id={}, parameter_id={}",
                             expression_id, unit_id, parameter_id))
            },
            ArtError::ChannelMismatch{ expected, actual } => {
                Some(format!("expected={}, actual={}", expected, actual))
            },
            ArtError::RateMismatch{ expected, actual } => {
                Some(format!("expected={:?}, actual={:?}", expected, actual))
            },
            ArtError::IoError { ref error } => {
                Some(format!("error={}", error))
            },
            ArtError::EncoderError { error } => {
                Some(format!("error={}", error))
            },
            ArtError::PortAudio { error } => {
                Some(format!("error={}", error))
            },
            _ => None
        }
    }
}

impl Error for ArtError {
    fn description(&self) -> &str {
        match *self {
            ArtError::UnimplementedOpcode { .. } => "Unimplemented opcode",
            ArtError::UndefinedUnit { .. } => "Undefined unit",
            ArtError::IndexError => "Index error",
            ArtError::UnitNotFound { .. } => "Unit not found",
            ArtError::ParameterNotFound { .. } => "Parameter not found",
            ArtError::ExpressionNotFound { .. } => "Expression not found",
            ArtError::ChannelMismatch { .. } => "Channel mismatch",
            ArtError::RateMismatch { .. } => "Rate mismatch",
            ArtError::InvalidByteCode => "Invalid byte code",
            ArtError::StackOverflow => "Stack overflow",
            ArtError::StackUnderflow => "Stack underflow",
            ArtError::BufferOverflow => "Buffer overflow",
            ArtError::InvalidStack => "Invalid stack",
            ArtError::IoError { .. } => "IO Error",
            ArtError::EncoderError { .. } => "Encoder error",
            ArtError::PortAudio { .. } => "PortAudio error"
        }
    }
}

impl FromError<PaError> for ArtError {
    fn from_error(error: PaError) -> ArtError {
        ArtError::PortAudio { error: error }
    }
}

impl FromError<io::Error> for ArtError {
    fn from_error(error: io::Error) -> ArtError {
        ArtError::IoError { error: error }
    }
}

impl FromError<EncoderError> for ArtError {
    fn from_error(error: EncoderError) -> ArtError {
        ArtError::EncoderError { error: error }
    }
}

impl fmt::Display for ArtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "{}", self.description());
        if let Some(detail) = self.detail() {
            let _ = write!(f, ": {}", detail);
        }
        let _ = write!(f, "\n");
        Ok(())
    }
}
