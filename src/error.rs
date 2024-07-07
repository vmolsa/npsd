#[doc(hidden)]
#[cfg(feature = "sync")]
use npsd_schema::SchemaInternal as Schema;

#[doc(hidden)]
#[cfg(feature = "async")]
use npsd_schema::AsyncSchemaInternal as AsyncSchema;

#[doc(hidden)]
use npsd_schema::InfoInternal as Info;

#[doc(hidden)]
use crate::PayloadInfo;

#[doc(hidden)]
use thiserror::Error as AsError;

#[cfg(feature = "sync")]
use crate::{Middleware, Payload, IntoPayload, FromPayload};

#[cfg(feature = "async")]
use crate::{AsyncMiddleware, AsyncPayload, AsyncIntoPayload, AsyncFromPayload};

#[cfg_attr(feature = "async", derive(AsyncSchema))]
#[cfg_attr(feature = "sync", derive(Schema))]
#[derive(Clone, Info, AsError, PartialEq, Debug)]
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
    Traced(String, String),

    #[error("External error: {0}")]
    External(String),
}
