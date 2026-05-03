//! Destroy message definitions

use core_3c::vector::Vector;

use crate::bytes_represented::{BytesRepresented, Error};

/// A message about destroying
pub struct DestroyMessage {
    /// A location is coordinates of triangle which on destroying building
    pub location: Vector,
}

impl BytesRepresented for DestroyMessage {
    fn encode(self) -> Vec<u8> {
        self.location.encode()
    }

    fn decode(decoder: &mut super::Decoder, bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Result::Ok(Self {
            location: Vector::decode(decoder, bytes)?,
        })
    }
}
