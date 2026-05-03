//! Build message definitions

use core_3c::vector::Vector;

use crate::bytes_represented::{BytesRepresented, Decoder, Error};

/// A message about building
pub struct BuildMessage {
    /// A location is coordinates of triangle which on sets building
    pub location: Vector,
    /// A build name is name of building from current game kit
    pub build_name: String,
}

impl BytesRepresented for BuildMessage {
    fn encode(self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();

        v.append(&mut self.location.encode());
        v.append(&mut self.build_name.encode());

        v
    }

    fn decode(decoder: &mut Decoder, bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Result::Ok(Self {
            location: Vector::decode(decoder, bytes)?,
            build_name: String::decode(decoder, bytes)?,
        })
    }
}
