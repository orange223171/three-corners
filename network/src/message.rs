//! Message definitions

use serde::{Deserialize, Serialize, Serializer};

use crate::message::{
    build::BuildMessage, destroy::DestroyMessage, grab::GrabMessage,
    set_triangle::SetTriangleMessage,
};

pub mod build;
pub mod destroy;
pub mod grab;
pub mod set_triangle;

/// Raw representation of message for sending and recieving
pub type RawMessage = [u8; 8192];

/// A network message
#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    VersionRequest,
    VersionResponce(u32, u32, u32),

    Build(BuildMessage),
    Destroy(DestroyMessage),
    Grab(GrabMessage),

    SetTriangle(SetTriangleMessage),

    Ok,
    OperationDenied,
}

impl Serializer for RawMessage {
    type Ok = Self;
}

impl Deserializer for RawMessage {
    type Ok = Self;
}
