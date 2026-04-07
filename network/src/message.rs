//! Message definitions

use core_3c::{building::Building, vector::Vector};

pub mod build;
pub mod destroy;
pub mod grab;
pub mod set_triangle;

/// Raw representation of message for sending and recieving
pub type RawMessage = [u8; 8192];

/// A network message
pub enum Message {
    VersionRequest,
    VersionResponce(u32, u32, u32),

    Build(Vector, String),
    Destroy(Vector),
    Grab(Vector),

    SetTriangle(Vector, Building),

    Ok,
    OperationDenied,
}
