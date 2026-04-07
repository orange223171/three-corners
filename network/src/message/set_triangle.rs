//! Set triangle message definitions

use core_3c::{board::Triangle, vector::Vector};
use serde::{Deserialize, Serialize};

pub struct SetTriangleMessage {
    pub location: Vector,
    pub triangle: Triangle,
}

impl Serialize for SetTriangleMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.location.x);
        serializer.serialize_u32(self.location.y)
    }
}

impl<'de> Deserialize<'de> for SetTriangleMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Result::Ok(Self {
            location: Vector { x: 0, y: 0 },
            triangle: None,
        })
    }
}
