use std::error::{Error, FromError};
use std::old_io::IoError;
use std::fmt;

use portaudio::pa::PaError;

use opcode::Opcode;

pub enum ArtError {
    UnimplementedOpcode { opcode: Opcode },
    UndefinedUnit { type_id: u32 },
    IndexError,
    ExpressionNotFound { expression_id: u32 },
    UnitNotFound { expression_id: u32, unit_id: u32 },
    ParameterNotFound { expression_id: u32, unit_id: u32, parameter_id: u32 },
    UnlinkedParameter { expression_id: u32, unit_id: u32, parameter_id: u32 },
    InvalidChannelCount,
    InvalidByteCode { error: Option<IoError> },
    StackOverflow,
    StackUnderflow,
    BufferOverflow,
    InvalidStack,
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
            ArtError::UnlinkedParameter { expression_id, unit_id,
                                          parameter_id } => {
                Some(format!("expression_id={}, unit_id={}, parameter_id={}",
                             expression_id, unit_id, parameter_id))
            },
            ArtError::InvalidByteCode { ref error } => {
                match error {
                    &Some(ref e) => Some(format!("error={}", e)),
                    &None => None
                }
            },
            // FIXME: Make PaError impl String
//            ArtError::PortAudio { error } => {
//                Some(format!("error={}", error))
//            },
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
            ArtError::UnlinkedParameter { .. } => "Unlinked parameter",
            ArtError::InvalidChannelCount => "Invalid channel count",
            ArtError::InvalidByteCode { .. } => "Invalid byte code",
            ArtError::StackOverflow => "Stack overflow",
            ArtError::StackUnderflow => "Stack underflow",
            ArtError::BufferOverflow => "Buffer overflow",
            ArtError::InvalidStack => "Invalid stack",
            ArtError::PortAudio { .. } => "PortAudio error"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ArtError::InvalidByteCode { ref error } => {
                match error {
                    &Some(ref e) => Some(e as &Error),
                    &None => None
                }
            },
            // FIXME: Make PaError impl Error
            //ArtError::PortAudio { error } => Some(&error as &Error),
            _ => None
        }
    }
}

impl FromError<PaError> for ArtError {
    fn from_error(error: PaError) -> ArtError {
        ArtError::PortAudio { error: error }
    }
}

impl FromError<IoError> for ArtError {
    fn from_error(error: IoError) -> ArtError {
        ArtError::InvalidByteCode { error: Some(error) }
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
