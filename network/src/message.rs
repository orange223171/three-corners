//! Message definitions

use core_3c::building::Building;

/// Raw message for send and recieve
pub type RawMessage = [u8; 8192];

/// Network message
pub enum Message {
    VersionRequest,
    VersionResponce(u32, u32, u32),

    Build(usize, usize, String),
    Destroy(usize, usize),
    Grab(usize, usize),

    UpdateTriangle(usize, usize, Building),
}
