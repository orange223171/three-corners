//! Error message definitions

use crate::bytes_represented::{BytesRepresented, Error};

const OPERATION_DENIED: u32 = 0;

/// An error message
#[repr(u32)]
#[derive(Debug)]
pub enum ErrorMessage {
    OperationDenied = OPERATION_DENIED,
}

impl BytesRepresented for ErrorMessage {
    fn encode(self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();

        match self {
            ErrorMessage::OperationDenied => v.append(&mut OPERATION_DENIED.encode()),
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

            _ => Result::Err(Error::UncorrectFormat(
                String::from("ErrorMessage"),
                value.to_be_bytes().to_vec(),
            )),
        }
    }
}
