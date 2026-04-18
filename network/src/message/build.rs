//! Build message definitions

use core_3c::vector::Vector;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildMessage {
    pub location: Vector,
    pub build_name: String,
}
