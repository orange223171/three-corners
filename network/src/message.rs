//! Message definitions

use serde::{Deserialize, Serialize};

use crate::message::{
    build::BuildMessage, destroy::DestroyMessage, grab::GrabMessage,
    set_triangle::SetTriangleMessage,
};

pub mod build;
pub mod destroy;
pub mod grab;
pub mod set_triangle;

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
