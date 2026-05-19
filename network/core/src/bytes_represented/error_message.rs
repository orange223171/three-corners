//! Error message definitions

use crate::bytes_represented::{BytesRepresented, Error};

const OPERATION_DENIED: u32 = 0;
const FAIL_TO_LOG_IN: u32 = 1;
const UNEXPECTED_MESSAGE: u32 = 2;

/// An error message
#[derive(Debug, Clone)]
pub enum ErrorMessage {
    OperationDenied,
    FailToLogIn,
    UnexpectedMessage,
}

impl BytesRepresented for ErrorMessage {
    fn encode(self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();

        match self {
            ErrorMessage::OperationDenied => v.append(&mut OPERATION_DENIED.encode()),
            ErrorMessage::FailToLogIn => v.append(&mut FAIL_TO_LOG_IN.encode()),
            ErrorMessage::UnexpectedMessage => v.append(&mut UNEXPECTED_MESSAGE.encode()),
        }

        v
    }

    fn decode(decoder: &mut super::Decoder, bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let value = u32::decode(decoder, bytes)?;
        match value {
            OPERATION_DENIED => Result::Ok(ErrorMessage::OperationDenied),

            FAIL_TO_LOG_IN => Result::Ok(ErrorMessage::FailToLogIn),

            UNEXPECTED_MESSAGE => Result::Ok(ErrorMessage::UnexpectedMessage),

            _ => Result::Err(Error::UncorrectFormat(
                String::from("ErrorMessage"),
                value.to_be_bytes().to_vec(),
            )),
        }
    }
}
