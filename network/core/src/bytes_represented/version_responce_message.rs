//!Version responce message definitions

use crate::bytes_represented::BytesRepresented;

#[derive(Debug)]
/// A responce with current version
pub struct VersionResponceMessage {
    /// Major version
    pub major: u32,
    /// Minor version
    pub minor: u32,
    /// Patch
    pub patch: u32,
}

impl BytesRepresented for VersionResponceMessage {
    fn encode(self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();

        v.append(&mut self.major.encode());
        v.append(&mut self.minor.encode());
        v.append(&mut self.patch.encode());

        v
    }

    fn decode(decoder: &mut super::Decoder, bytes: &[u8]) -> Result<Self, super::Error>
    where
        Self: Sized,
    {
        Result::Ok(Self {
            major: u32::decode(decoder, bytes)?,
            minor: u32::decode(decoder, bytes)?,
            patch: u32::decode(decoder, bytes)?,
        })
    }
}
