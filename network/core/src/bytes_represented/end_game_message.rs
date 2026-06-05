//! End game message definitions

use crate::bytes_represented::BytesRepresented;

/// A message signalling the end of the game with the winner
#[derive(Debug, Clone)]
pub struct EndGameMessage {
    /// The winner's player name
    pub player: String,
}

impl BytesRepresented for EndGameMessage {
    fn encode(self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        v.append(&mut self.player.encode());
        v
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
