//! Player's state message definitions

use core_3c::player_state::PlayerState;

use crate::bytes_represented::BytesRepresented;

/// A message with player state
#[derive(Debug, Clone)]
pub struct PlayerStateMessage {
    /// A player
    pub player: String,
    /// Player's state
    pub state: PlayerState,
}

impl BytesRepresented for PlayerStateMessage {
    fn encode(self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();

        v.append(&mut self.player.encode());
        v.append(&mut self.state.encode());

        v
    }

    fn decode(decoder: &mut super::Decoder, bytes: &[u8]) -> Result<Self, super::Error>
    where
        Self: Sized,
    {
        Result::Ok(Self {
            player: String::decode(decoder, bytes)?,
            state: PlayerState::decode(decoder, bytes)?,
        })
    }
}
