//! Add 2FA responce message's definitions

use crate::bytes_represented::BytesRepresented;

#[derive(Debug, Clone)]
pub struct Add2faResponceMessage {
    pub secret: String,
}

impl BytesRepresented for Add2faResponceMessage {
    fn encode(self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();

        v.append(&mut self.secret.encode());

        v
    }

    fn decode(decoder: &mut super::Decoder, bytes: &[u8]) -> Result<Self, super::Error>
    where
        Self: Sized,
    {
        Result::Ok(Self {
            secret: String::decode(decoder, bytes)?,
        })
    }
}
