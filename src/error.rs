use npsd_schema::SchemaInternal as Schema;
#[doc(hidden)]
use thiserror::Error as AsError;

use crate::{PayloadContext, Middleware, PayloadInfo, IntoPayload, FromPayload, Payload, PayloadHandler, PayloadConstHash};

#[derive(Schema, Clone, AsError, PartialEq, Debug)]
pub enum Error {
    #[error("Invalid length: expected `{expected}`, found `{found}`")]
    InvalidLength {
        expected: usize,
        found: usize,
    },

    #[error("Invalid UTF-8 sequence")]
    InvalidUtf8(String),

    #[error("Unknown variant `{0}`")]
    UnknownVariant(String),

    #[error("Index out of bounds: `{0}`")]
    IndexOutOfBounds(usize),

    #[error("Reached max depth of nested types for: `{0}`")]
    NestedDepthLimit(String),

    #[error("Failed to upgrade Weak reference")]
    WeakUpgrade,

    #[error("Invalid pointer")]
    NullPtr,

    #[error("Invalid Time: `{0}`")]
    Time(String),

    #[error("Traced error: `{0}`, path: `{1}`")]
    Traced(String, String)
}
