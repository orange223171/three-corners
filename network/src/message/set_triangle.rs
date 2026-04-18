//! Set triangle message definitions

use core_3c::{board::Triangle, vector::Vector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SetTriangleMessage {
    pub location: Vector,
    pub triangle: Triangle,
}
