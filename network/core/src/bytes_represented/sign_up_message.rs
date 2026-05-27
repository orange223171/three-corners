//! Register message definitions

use crate::bytes_represented::BytesRepresented;

/// A register message
#[derive(Debug, Clone)]
pub struct SignUpMessage {
    /// A player
    pub player: String,
    /// A password
    pub password: String,
}

impl BytesRepresented for SignUpMessage {
    fn encode(self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();

        v.append(&mut self.player.encode());
        v.append(&mut self.password.encode());

        v
    }

    fn decode(decoder: &mut super::Decoder, bytes: &[u8]) -> Result<Self, super::Error>
    where
        Self: Sized,
    {
        Result::Ok(Self {
            player: String::decode(decoder, bytes)?,
            password: String::decode(decoder, bytes)?,
        })
    }
}
