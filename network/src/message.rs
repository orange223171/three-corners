//! Message definitions

use crate::bytes_represented::{
    BytesRepresented, Error, build::BuildMessage, destroy::DestroyMessage, grab::GrabMessage,
    set_triangle::SetTriangleMessage,
};

/// A network message
#[derive(Debug)]
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

impl Message {
    pub fn as_bytes(self) -> Vec<u8> {
        todo!()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        todo!()
    }
}
