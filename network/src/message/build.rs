//! Build message definitions

use core_3c::vector::Vector;
use serde::{Deserialize, Serialize};

pub struct BuildMessage {
    pub location: Vector,
    pub build_name: String,
}

impl Serialize for BuildMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.location.x);
        serializer.serialize_u32(self.location.y);

        serializer.serialize_str(&self.build_name.as_str())
    }
}

impl<'de> Deserialize<'de> for BuildMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Result::Ok(Self {
            location: Vector { x: 0, y: 0 },
            build_name: String::from("field"),
        })
    }
}
