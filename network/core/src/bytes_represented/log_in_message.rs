//! Log in message definitions

use crate::bytes_represented::BytesRepresented;

/// A log in message
#[derive(Debug, Clone)]
pub struct LogInMessage {
    /// A player
    pub player: String,
}

impl BytesRepresented for LogInMessage {
    fn encode(self) -> Vec<u8> {
        self.player.encode()
    }

    fn decode(decoder: &mut super::Decoder, bytes: &[u8]) -> Result<Self, super::Error>
    where
        Self: Sized,
    {
        Result::Ok(Self {
            player: String::decode(decoder, bytes)?,
        })
    }
}
