//! Grab message definitions

use core_3c::vector::Vector;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GrabMessage {
    location: Vector,
}
