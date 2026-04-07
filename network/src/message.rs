//! Message definitions

use core_3c::{building::Building, vector::Vector};

/// Raw message for send and recieve
pub type RawMessage = [u8; 8192];

/// Network message
pub enum Message {
    VersionRequest,
    VersionResponce(u32, u32, u32),

    Build(Vector, String),
    Destroy(Vector),
    Grab(Vector),

    SetTriangle(Vector, Building),
}
