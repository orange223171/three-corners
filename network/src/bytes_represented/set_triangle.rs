//! Set triangle message definitions

use core_3c::{board::Triangle, building::Building, vector::Vector};

use crate::bytes_represented::{BytesRepresented, Error};

pub struct SetTriangleMessage {
    pub location: Vector,
    pub triangle: Triangle,
}

impl BytesRepresented for SetTriangleMessage {
    fn encode(self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();

        v.append(&mut self.location.encode());

        match self.triangle {
            None => v.append(&mut String::from("empty").encode()),
            Some(building) => {
                v.append(&mut String::from("building").encode());
                v.append(&mut building.encode())
            }
        }

        v
    }

    fn decode(decoder: &mut super::Decoder, bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Result::Ok(Self {
            location: Vector::decode(decoder, bytes)?,
            triangle: {
                let content = String::decode(decoder, bytes)?;

                if content == "empty" {
                    None
                } else if content == "building" {
                    Some(Building::decode(decoder, bytes)?)
                } else {
                    return Result::Err(Error::UncorrectFormat(
                        String::from("triangle content"),
                        content.as_bytes().to_vec(),
                    ));
                }
            },
        })
    }
}
