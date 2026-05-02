//! Build message definitions

use core_3c::vector::Vector;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildMessage {
    pub location: Vector,
    pub build_name: String,
}

impl BuildMessage {
    pub fn as_bytes(&mut self) -> Vec<u8> {
        let mut v = vec![];

        v
    }

    pub fn from_bytes(bytes: &[u8]) -> BuildMessage {
        BuildMessage {
            location: Vector { x: 0, y: 0 },
            build_name: String::from(""),
        }
    }
}
