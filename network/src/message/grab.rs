//! Grab message definitions

use core_3c::vector::Vector;
use serde::{Deserialize, Serialize};

pub struct GrabMessage {
    location: Vector,
}

impl Serialize for GrabMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.location.x);
        serializer.serialize_u32(self.location.y)
    }
}

impl<'de> Deserialize<'de> for GrabMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Result::Ok(Self {
            location: Vector { x: 0, y: 0 },
        })
    }
}
