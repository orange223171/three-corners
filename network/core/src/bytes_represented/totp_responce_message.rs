//! TOTP eraponce message's definitons

use crate::bytes_represented::BytesRepresented;

#[derive(Debug, Clone)]
pub struct TotpResponceMessage {
    pub totp_code: String,
}

impl BytesRepresented for TotpResponceMessage {
    fn encode(self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();

        v.append(&mut self.totp_code.encode());

        v
    }

    fn decode(decoder: &mut super::Decoder, bytes: &[u8]) -> Result<Self, super::Error>
    where
        Self: Sized,
    {
        Result::Ok(Self {
            totp_code: String::decode(decoder, bytes)?,
        })
    }
}
