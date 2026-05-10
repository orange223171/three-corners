//! Message definitions

use crate::bytes_represented::{
    BytesRepresented, Decoder, Error, build_message::BuildMessage, destroy_message::DestroyMessage,
    error_message::ErrorMessage, grab_message::GrabMessage, log_in_message::LogInMessage,
    set_triangle_message::SetTriangleMessage, version_responce_message::VersionResponceMessage,
};

const OK_MESSAGE: u32 = 0;
const ERROR_MESSAGE: u32 = 1;

const VERSION_REQUEST_MESSAGE: u32 = 2;
const VERSION_RESPONCE_MESSAGE: u32 = 3;

const LOG_IN_MESSAGE: u32 = 8;

const BUILD_MESSAGE: u32 = 16;
const DESTROY_MESSAGE: u32 = 17;
const GRAB_MESSAGE: u32 = 18;

const SET_TRIANGLE_MESSAGE: u32 = 64;

/// A network message
pub enum Message {
    Ok,
    Error(ErrorMessage),

    LogIn(LogInMessage),

    VersionRequest,
    VersionResponce(VersionResponceMessage),

    Build(BuildMessage),
    Destroy(DestroyMessage),
    Grab(GrabMessage),

    SetTriangle(SetTriangleMessage),
}

impl Message {
    pub fn as_bytes(self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();

        match self {
            Message::Ok => v.append(&mut OK_MESSAGE.encode()),
            Message::Error(error_message) => {
                v.append(&mut ERROR_MESSAGE.encode());
                v.append(&mut error_message.encode());
            }

            Message::LogIn(log_in_message) => {
                v.append(&mut LOG_IN_MESSAGE.encode());
                v.append(&mut log_in_message.encode());
            }

            Message::VersionRequest => v.append(&mut VERSION_REQUEST_MESSAGE.encode()),
            Message::VersionResponce(version_responce_message) => {
                v.append(&mut VERSION_RESPONCE_MESSAGE.encode());
                v.append(&mut version_responce_message.encode());
            }
            Message::Build(build_message) => {
                v.append(&mut BUILD_MESSAGE.encode());
                v.append(&mut build_message.encode());
            }
            Message::Destroy(destroy_message) => {
                v.append(&mut DESTROY_MESSAGE.encode());
                v.append(&mut destroy_message.encode());
            }
            Message::Grab(grab_message) => {
                v.append(&mut GRAB_MESSAGE.encode());
                v.append(&mut grab_message.encode());
            }
            Message::SetTriangle(set_triangle_message) => {
                v.append(&mut SET_TRIANGLE_MESSAGE.encode());
                v.append(&mut set_triangle_message.encode());
            }
        }

        v
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let mut decoder = Decoder::new();
        let value = u32::decode(&mut decoder, bytes)?;

        match value {
            OK_MESSAGE => Result::Ok(Message::Ok),
            ERROR_MESSAGE => Result::Ok(Message::Error(ErrorMessage::decode(&mut decoder, bytes)?)),

            VERSION_REQUEST_MESSAGE => Result::Ok(Message::VersionRequest),
            VERSION_RESPONCE_MESSAGE => Result::Ok(Message::VersionResponce(
                VersionResponceMessage::decode(&mut decoder, bytes)?,
            )),

            LOG_IN_MESSAGE => {
                Result::Ok(Message::LogIn(LogInMessage::decode(&mut decoder, bytes)?))
            }

            BUILD_MESSAGE => Result::Ok(Message::Build(BuildMessage::decode(&mut decoder, bytes)?)),
            DESTROY_MESSAGE => Result::Ok(Message::Destroy(DestroyMessage::decode(
                &mut decoder,
                bytes,
            )?)),
            GRAB_MESSAGE => Result::Ok(Message::Grab(GrabMessage::decode(&mut decoder, bytes)?)),

            SET_TRIANGLE_MESSAGE => Result::Ok(Message::SetTriangle(SetTriangleMessage::decode(
                &mut decoder,
                bytes,
            )?)),

            _ => Result::Err(Error::UncorrectFormat(
                String::from("Message"),
                value.to_be_bytes().to_vec(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_ok_encode() {
        let message = Message::Ok;
        let bytes: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00];

        assert_eq!(message.as_bytes(), bytes)
    }

    #[test]
    fn message_ok_decode() {
        let bytes: [u8; 4] = [0x00, 0x00, 0x00, 0x00];

        match Message::from_bytes(&bytes).expect("wrong message") {
            Message::Ok => assert!(true),
            _ => assert!(false),
        }
    }
}
