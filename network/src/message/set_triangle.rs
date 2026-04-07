//! Set triangle message definitions

use core_3c::{board::Triangle, vector::Vector};

pub struct SetTriangleMessage {
    pub location: Vector,
    pub triangle: Triangle,
}
