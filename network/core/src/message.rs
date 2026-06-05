//! Message definitions

use crate::{
    bytes_represented::{
        BytesRepresented, Decoder, Error, add_2fa_responce_message::Add2faResponceMessage,
        build_message::BuildMessage, destroy_message::DestroyMessage,
        end_game_message::EndGameMessage, error_message::ErrorMessage, grab_message::GrabMessage,
        log_in_message::LogInMessage, player_state_message::PlayerStateMessage,
        set_triangle_message::SetTriangleMessage, sign_up_message::SignUpMessage,
        totp_responce_message::TotpResponceMessage,
        version_responce_message::VersionResponceMessage,
    },
    message::Message::Remove2fa,
};

const OK_MESSAGE: u32 = 0;
const ERROR_MESSAGE: u32 = 1;

const VERSION_REQUEST_MESSAGE: u32 = 2;
const VERSION_RESPONCE_MESSAGE: u32 = 3;

const LOG_IN_MESSAGE: u32 = 8;
const SIGN_UP_MESSAGE: u32 = 9;

const LOG_IN_SUCCESSFUL_MESSAGE: u32 = 10;
const SIGN_UP_SUCCESSFUL_MESSAGE: u32 = 11;

const TOTP_REQUEST_MESSAGE: u32 = 16;
const TOTP_RESPONCE_MESSAGE: u32 = 17;
const ADD_2FA_REQUEST_MESSAGE: u32 = 18;
const ADD_2FA_RESPONCE_MESSAGE: u32 = 19;
const REMOVE_2FA_MESSAGE: u32 = 20;

const BUILD_MESSAGE: u32 = 32;
const DESTROY_MESSAGE: u32 = 33;
const GRAB_MESSAGE: u32 = 34;

const SET_TRIANGLE_MESSAGE: u32 = 64;
const PLAYER_STATE_MESSAGE: u32 = 65;
const GAME_DATA_REQUEST_MESSAGE: u32 = 66;
const END_GAME_MESSAGE: u32 = 67;

/// A network message
#[derive(Debug, Clone)]
pub enum Message {
    Ok,
    Error(ErrorMessage),

    LogIn(LogInMessage),
    SignUp(SignUpMessage),

    LogInSuccessful,
    SignUpSuccessful,

    TotpRequest,
    TotpResponce(TotpResponceMessage),

    Add2faRequest,
    Add2faResponce(Add2faResponceMessage),
    Remove2fa,

    VersionRequest,
    VersionResponce(VersionResponceMessage),

    Build(BuildMessage),
    Destroy(DestroyMessage),
    Grab(GrabMessage),

    SetTriangle(SetTriangleMessage),
    PlayerState(PlayerStateMessage),
    GameDataRequest,
    EndGame(EndGameMessage),
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
            Message::SignUp(register_message) => {
                v.append(&mut SIGN_UP_MESSAGE.encode());
                v.append(&mut register_message.encode());
            }

            Message::LogInSuccessful => {
                v.append(&mut LOG_IN_SUCCESSFUL_MESSAGE.encode());
            }
            Message::SignUpSuccessful => {
                v.append(&mut SIGN_UP_SUCCESSFUL_MESSAGE.encode());
            }

            Message::TotpRequest => {
                v.append(&mut TOTP_REQUEST_MESSAGE.encode());
            }
            Message::TotpResponce(totp_responce_message) => {
                v.append(&mut TOTP_RESPONCE_MESSAGE.encode());
                v.append(&mut totp_responce_message.encode());
            }

            Message::Add2faRequest => {
                v.append(&mut ADD_2FA_REQUEST_MESSAGE.encode());
            }
            Message::Add2faResponce(add_2fa_responce_message) => {
                v.append(&mut ADD_2FA_RESPONCE_MESSAGE.encode());
                v.append(&mut add_2fa_responce_message.encode());
            }
            Message::Remove2fa => {
                v.append(&mut REMOVE_2FA_MESSAGE.encode());
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
            Message::PlayerState(player_state_message) => {
                v.append(&mut PLAYER_STATE_MESSAGE.encode());
                v.append(&mut player_state_message.encode());
            }
            Message::GameDataRequest => {
                v.append(&mut GAME_DATA_REQUEST_MESSAGE.encode());
            }
            Message::EndGame(end_game_message) => {
                v.append(&mut END_GAME_MESSAGE.encode());
                v.append(&mut end_game_message.encode());
            }
        }

        v.append(&mut vec![0; 8192 - v.len()]);

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
            SIGN_UP_MESSAGE => {
                Result::Ok(Message::SignUp(SignUpMessage::decode(&mut decoder, bytes)?))
            }

            LOG_IN_SUCCESSFUL_MESSAGE => Result::Ok(Message::LogInSuccessful),
            SIGN_UP_SUCCESSFUL_MESSAGE => Result::Ok(Message::SignUpSuccessful),

            TOTP_REQUEST_MESSAGE => Result::Ok(Message::TotpRequest),
            TOTP_RESPONCE_MESSAGE => Result::Ok(Message::TotpResponce(
                TotpResponceMessage::decode(&mut decoder, bytes)?,
            )),

            ADD_2FA_REQUEST_MESSAGE => Result::Ok(Message::Add2faRequest),
            ADD_2FA_RESPONCE_MESSAGE => Result::Ok(Message::Add2faResponce(
                Add2faResponceMessage::decode(&mut decoder, bytes)?,
            )),
            REMOVE_2FA_MESSAGE => Result::Ok(Message::Remove2fa),

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
            PLAYER_STATE_MESSAGE => Result::Ok(Message::PlayerState(PlayerStateMessage::decode(
                &mut decoder,
                bytes,
            )?)),
            GAME_DATA_REQUEST_MESSAGE => Result::Ok(Message::GameDataRequest),
            END_GAME_MESSAGE => Result::Ok(Message::EndGame(EndGameMessage::decode(
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
