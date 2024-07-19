//! # npsd (Network Payload Serializer / Deserializer)
//!
//! The `npsd` crate provides a flexible and efficient way to serialize and deserialize network payloads. 
//! It supports converting Rust types into byte streams suitable for network transmission and reconstructing 
//! those types from byte streams received over the network. This is particularly useful for networked 
//! applications that require efficient and reliable data exchange.
//!
//! ## Features
//! - Serialize and deserialize complex Rust types to and from byte streams.
//! - Support for custom serialization contexts.
//! - Middleware support for extensible processing during serialization/deserialization.
//!
//! ## Examples
//! ### Sync Schema
//! Requires the `sync` feature to be enabled.
//! ```rust
//! # #[cfg(feature = "sync")]
//! use npsd::{Payload, Schema, Next, Info};
//!
//! # #[cfg(feature = "sync")]
//! #[derive(Schema, Info, PartialEq, Debug)]
//! enum Animal {
//!     Dog,
//!     Frog(String, Vec<isize>),
//!     Cat { age: usize, name: String },
//!     AntHive(Vec<String>),
//! }
//!
//! # #[cfg(feature = "sync")]
//! #[test]
//! fn test_schema() {
//!     // Create Middleware
//!     let mut next = Next::default();
//! 
//!     // Create an instance of `Animal`.
//!     let animal = Animal::Frog("Frog".to_string(), vec![12393818, -19383812, 11111, -1093838482]);
//!
//!     // Serialize the `animal` instance into a packet.
//!     animal.into_packet(&mut (), &mut next).unwrap();
//!
//!     // Create a copy of serialized data if needed
//!     let _serialized = next.serialized();
//!
//!     // Deserialize the packet back into an `Animal` instance.
//!     let deserialized = Animal::from_packet(&mut (), &mut next).unwrap();
//!
//!     // Ensure the deserialized instance matches the original.
//!     assert_eq!(deserialized, animal);
//! }
//! ```
//!
//! ### Async Schema
//! Requires the `async` feature to be enabled.
//! ```rust
//! # #[cfg(feature = "async")]
//! use npsd::{AsyncPayload, AsyncSchema, Next, Info};
//!
//! # #[cfg(feature = "async")]
//! #[derive(AsyncSchema, Info, PartialEq, Debug)]
//! enum Animal {
//!     Dog,
//!     Frog(String, Vec<isize>),
//!     Cat { age: usize, name: String },
//!     AntHive(Vec<String>),
//! }
//!
//! # #[cfg(feature = "async")]
//! #[tokio::test]
//! async fn test_schema() {
//!     // Create Middleware
//!     let mut next = Next::default();
//!
//!     // Create an instance of `Animal`.
//!     let animal = Animal::Frog("Frog".to_string(), vec![12393818, -19383812, 11111, -1093838482]);
//!
//!     // Serialize the `animal` instance into a packet.
//!     animal.poll_into_packet(&mut (), &mut next).await.unwrap();
//!
//!     // Create a copy of serialized data if needed
//!     let _serialized = next.serialized();
//!
//!     // Deserialize the packet back into an `Animal` instance.
//!     let deserialized = Animal::poll_from_packet(&mut (), &mut next).await.unwrap();
//!
//!     // Ensure the deserialized instance matches the original.
//!     assert_eq!(deserialized, animal);
//! }
//! ```
//!
//! ### Sync Bitmap
//! Requires the `sync` feature to be enabled.
//! ```rust
//! # #[cfg(feature = "sync")]
//! use npsd::{Payload, Bitmap, Next, Info};
//!
//! # #[cfg(feature = "sync")]
//! #[derive(Bitmap, Info, PartialEq, Debug)]
//! struct Flags {
//!    a: bool,
//!    b: bool,
//!    c: bool,
//! }
//!
//! # #[cfg(feature = "sync")]
//! #[test]
//! fn test_bitmap() {
//!    // Create Middleware
//!    let mut next = Next::default();
//!
//!    // Create an u8 bitmap of `Flags`.
//!    let flags = Flags { a: true, b: false, c: true };
//!
//!    // Serialize the `Flags` into a packet.
//!    flags.into_packet(&mut (), &mut next).unwrap();
//!
//!    // Create a copy of serialized data if needed
//!    let _serialized = next.serialized();
//!
//!    // Deserialize the packet back into an `Flags`.
//!    let deserialized = Flags::from_packet(&mut (), &mut next).unwrap();
//!
//!    // Ensure the deserialized matches the original.
//!    assert_eq!(deserialized, flags);
//! }
//! ```
//!
//! ### Async Bitmap
//! Requires the `async` feature to be enabled.
//! ```rust
//! # #[cfg(feature = "async")]
//! use npsd::{AsyncPayload, AsyncBitmap, Next, Info};
//!
//! # #[cfg(feature = "async")]
//! #[derive(AsyncBitmap, Info, PartialEq, Debug)]
//! struct Flags {
//!    a: bool,
//!    b: bool,
//!    c: bool,
//! }
//!
//! # #[cfg(feature = "async")]
//! #[tokio::test]
//! async fn test_async_bitmap() {
//!    // Create Middleware
//!    let mut next = Next::default();
//!
//!    // Create an u8 bitmap of `Flags`.
//!    let flags = Flags { a: true, b: false, c: true };
//!
//!    // Serialize the `Flags` into a packet.
//!    flags.poll_into_packet(&mut (), &mut next).await.unwrap();
//!
//!    // Create a copy of serialized data if needed
//!    let _serialized = next.serialized();
//!
//!    // Deserialize the packet back into an `Flags`.
//!    let deserialized = Flags::poll_from_packet(&mut (), &mut next).await.unwrap();
//!
//!    // Ensure the deserialized matches the original.
//!    assert_eq!(deserialized, flags);
//! }
//! ```


/// The `Middleware` trait defines methods for converting types to and from payloads of bytes.
///
/// ### Methods
///
/// - `fn into_payload<C, T: IntoPayload<C>>(&mut self, value: &T, ctx: &mut C) -> Result<(), Error>`:
///     - Converts a value into a payload of bytes. This method takes a value and a context, serializes the value into a byte stream, and writes the resulting bytes into the handler.
/// - `fn from_payload<'a, C, T: FromPayload<'a, C>>(&mut self, ctx: &mut C) -> Result<T, Error>`:
///     - Converts a payload of bytes back into a value. This method reads bytes from the handler, uses the context to interpret them, and reconstructs the original value.
/// - `fn write<T>(&mut self, data: &[T]) -> Result<(), Error>`:
///     - Writes raw data into the handler. This method takes a slice of data and appends it to the handler after ensuring that the size of the data elements is 1 byte.
/// - `fn read<'a, T>(&'a mut self, nbytes: usize) -> Result<&'a [T], Error>`:
///     - Reads raw data from the handler. This method reads a specified number of bytes from the handler, splits the handler's data accordingly, and returns a slice of the read data.
/// - `fn read_mut<'a, T>(&'a mut self, nbytes: usize) -> Result<&'a mut [T], Error>`:
///     - Reads raw data from the handler. This method reads a specified number of bytes from the handler, splits the handler's data accordingly, and returns a mutable slice of the read data.
/// - `fn push<T: AnyBox<'a>>(&mut self, value: Box<T>) -> Result<&'a T, Error>`:
///     - Pushes a boxed value into the handler, returning a reference to the stored value.
/// - `fn push_mut<T: AnyBox<'a>>(&mut self, value: Box<T>) -> Result<&'a mut T, Error>`:
///     - Pushes a boxed value into the handler, returning a mutable reference to the stored value.
/// - `fn push_array<T: AnyBox<'a>>(&mut self, values: Box<[T]>) -> Result<&'a [T], Error>`:
///     - Pushes a boxed array of values into the handler, returning a reference to the stored array.
/// - `fn push_array_mut<T: AnyBox<'a>>(&mut self, values: Box<[T]>) -> Result<&'a mut [T], Error>`:
///     - Pushes a boxed array of values into the handler, returning a mutable reference to the stored array.
#[cfg(feature = "sync")]
pub trait Middleware<'a> {
    fn into_payload<C, T: IntoPayload<C>>(&mut self, value: &T, ctx: &mut C) -> Result<(), Error>;
    fn from_payload<C, T: FromPayload<'a, C>>(&mut self, ctx: &mut C) -> Result<T, Error>;

    fn write<T>(&mut self, data: &[T]) -> Result<(), Error>;
    fn read<T>(&mut self, nbytes: usize) -> Result<&'a [T], Error>;
    fn read_mut<T>(&mut self, nbytes: usize) -> Result<&'a mut [T], Error>;

    fn push<T: AnyBox<'a>>(&mut self, value: Box<T>) -> Result<&'a T, Error>;
    fn push_mut<T: AnyBox<'a>>(&mut self, value: Box<T>) -> Result<&'a mut T, Error>;
    fn push_array<T: AnyBox<'a>>(&mut self, values: Box<[T]>) -> Result<&'a [T], Error>;
    fn push_array_mut<T: AnyBox<'a>>(&mut self, values: Box<[T]>) -> Result<&'a mut [T], Error>;
}

/// The `AsyncMiddleware` trait defines asynchronous methods for converting types to and from payloads of bytes.
///
/// ### Methods
/// - `fn poll_into_payload<'a, C, T: AsyncIntoPayload<C>>(&mut self, value: &T, ctx: &mut C) -> impl Future<Output = Result<(), Error>>`:
///     - Polls the conversion of a value into a payload of bytes asynchronously.
/// - `fn poll_from_payload<'a, C, T: AsyncFromPayload<'a, C>>(&mut self, ctx: &mut C) -> impl Future<Output = Result<T, Error>>`:
///     - Polls the conversion of a payload of bytes back into a value asynchronously.
/// - `fn poll_write<T>(&mut self, data: &[T]) -> impl Future<Output = Result<(), Error>>`:
///     - Polls the asynchronous writing of raw data into the handler.
/// - `fn poll_read<'a, T>(&'a mut self, nbytes: usize) -> impl Future<Output = Result<&'a [T], Error>>`:
///     - Polls the asynchronous reading of raw data from the handler.
/// - `fn poll_read_mut<'a, T>(&'a mut self, nbytes: usize) -> impl Future<Output = Result<&'a mut [T], Error>>`:
///     - Polls the asynchronous reading of raw data from the handler, returning a mutable slice of the read data.
/// - `fn poll_push<T: AnyBox<'a>>(&mut self, value: Box<T>) -> impl Future<Output = Result<&'a T, Error>>`:
///     - Polls the asynchronous pushing of a boxed value into the handler, returning a reference to the stored value.
/// - `fn poll_push_mut<T: AnyBox<'a>>(&mut self, value: Box<T>) -> impl Future<Output = Result<&'a mut T, Error>>`:
///     - Polls the asynchronous pushing of a boxed value into the handler, returning a mutable reference to the stored value.
/// - `fn poll_push_array<T: AnyBox<'a>>(&mut self, values: Box<[T]>) -> impl Future<Output = Result<&'a [T], Error>>`:
///     - Polls the asynchronous pushing of a boxed array of values into the handler, returning a reference to the stored array.
/// - `fn poll_push_array_mut<T: AnyBox<'a>>(&mut self, values: Box<[T]>) -> impl Future<Output = Result<&'a mut [T], Error>>`:
///     - Polls the asynchronous pushing of a boxed array of values into the handler, returning a mutable reference to the stored array.
#[cfg(feature = "async")]
pub trait AsyncMiddleware<'a>: Send + Sync {
    fn poll_into_payload<C: Send + Sync, T: AsyncIntoPayload<C>>(&mut self, value: &T, ctx: &mut C) -> impl Future<Output = Result<(), Error>>;
    fn poll_from_payload<C: Send + Sync, T: AsyncFromPayload<'a, C>>(&mut self, ctx: &mut C) -> impl Future<Output = Result<T, Error>>;

    fn poll_write<T>(&mut self, data: &[T]) -> impl Future<Output = Result<(), Error>>;
    fn poll_read<T: 'a>(&mut self, nbytes: usize) -> impl Future<Output = Result<&'a [T], Error>>;
    fn poll_read_mut<T: 'a>(&mut self, nbytes: usize) -> impl Future<Output = Result<&'a mut [T], Error>>;

    fn poll_push<T: AnyBox<'a>>(&mut self, value: Box<T>) -> impl Future<Output = Result<&'a T, Error>>;
    fn poll_push_mut<T: AnyBox<'a>>(&mut self, value: Box<T>) -> impl Future<Output = Result<&'a mut T, Error>>;
    fn poll_push_array<T: AnyBox<'a>>(&mut self, values: Box<[T]>) -> impl Future<Output = Result<&'a [T], Error>>;
    fn poll_push_array_mut<T: AnyBox<'a>>(&mut self, values: Box<[T]>) -> impl Future<Output = Result<&'a mut [T], Error>>;
}

/// The `IntoPayload` trait is used to convert a type into a payload of bytes.
///
/// ### Methods
/// - `fn into_payload<'m, M: Middleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error>`:
///     - Converts a value into a payload of bytes. This method takes the value, context, and middleware, serializes the value into a byte stream, and writes it into the handler.
#[cfg(not(feature = "info"))]
#[cfg(feature = "sync")]
pub trait IntoPayload<C> {
    fn into_payload<'m, M: Middleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error>;
}

#[cfg(feature = "info")]
#[cfg(feature = "sync")]
pub trait IntoPayload<C>: PayloadInfo {
    fn into_payload<'m, M: Middleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error>;
}

/// The `AsyncIntoPayload` trait is used for asynchronous methods for converting types into payloads of bytes.
///
/// ### Methods
/// - `fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> impl Future<Output = Result<(), Error>>`:
///     - Polls the conversion of a value into a payload of bytes asynchronously.
#[cfg(not(feature = "info"))]
#[cfg(feature = "async")]
pub trait AsyncIntoPayload<C: Send + Sync>: Send + Sync {
    fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> impl Future<Output = Result<(), Error>>;
}

#[cfg(feature = "info")]
#[cfg(feature = "async")]
pub trait AsyncIntoPayload<C: Send + Sync>: PayloadInfo + Send + Sync {
    fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> impl Future<Output = Result<(), Error>>;
}

/// The `FromPayload` trait is used to convert a payload of bytes back into a type.
/// 
/// ### Methods
/// - `fn from_payload<M: Middleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error>`:
///     - Converts a payload of bytes back into a value. This method reads bytes from the handler, uses the context and middleware to interpret them, and reconstructs the original value.
#[cfg(not(feature = "info"))]
#[cfg(feature = "sync")]
pub trait FromPayload<'a, C>: Sized {
    fn from_payload<M: Middleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error>;
}

#[cfg(feature = "info")]
#[cfg(feature = "sync")]
pub trait FromPayload<'a, C>: PayloadInfo + Sized {
    fn from_payload<M: Middleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error>;
}

/// The `AsyncFromPayload` trait is used for asynchronous methods for converting payloads of bytes back into types.
///
/// ### Methods
/// - `fn poll_from_payload<'m, M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> impl Future<Output = Result<Self, Error>>`:
///     - Polls the conversion of a payload of bytes back into a value asynchronously.
#[cfg(not(feature = "info"))]
#[cfg(feature = "async")]
pub trait AsyncFromPayload<'a, C: Send + Sync>: Sized + Send + Sync {
    fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> impl Future<Output = Result<Self, Error>>;
}

#[cfg(feature = "info")]
#[cfg(feature = "async")]
pub trait AsyncFromPayload<'a, C: Send + Sync>: PayloadInfo + Sized + Send + Sync {
    fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> impl Future<Output = Result<Self, Error>>;
}

/// The `Payload` trait combines `IntoPayload` and `FromPayload` to facilitate complete serialization and deserialization of types.
///
/// ### Methods
/// - `fn into_packet<'b, M: Middleware<'b>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error>`:
///     - Serializes a value into a buffer. This method takes the value, context, and an initial buffer capacity, serializes the value, and returns the resulting byte buffer.
/// - `fn from_packet<'m, M: Middleware<'m>>(ctx: &mut C, next: &mut M) -> Result<Self, Error>`:
///     - Deserializes a buffer into a value. This method takes a context and a buffer containing the serialized data, and returns the deserialized value.
#[cfg(feature = "sync")]
pub trait Payload<'a, C>: IntoPayload<C> + FromPayload<'a, C> + Sized {
    fn into_packet<'b, M: Middleware<'b>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(self, ctx)
    }

    #[inline(always)]
    fn from_packet<M: Middleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        next.from_payload(ctx)
    }
}

/// The `AsyncPayload` trait combines `AsyncIntoPayload` and `AsyncFromPayload` to asynchronous methods for complete serialization and deserialization of types.
///
/// ### Methods
/// - `fn poll_into_packet<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> impl Future<Output = Result<(), Error>>`:
///     - Initiates the asynchronous conversion of a value into a packet.
/// - `fn poll_from_packet<'m, M: AsyncMiddleware<'m>>(ctx: &mut C, next: &mut M) -> impl Future<Output = Result<Self, Error>>`:
///     - Initiates the asynchronous deserialization of a packet into a value.
#[cfg(feature = "async")]
pub trait AsyncPayload<'a, C: Send + Sync>: AsyncIntoPayload<C> + AsyncFromPayload<'a, C> + 'a + Send + Sync + Sized {
    fn poll_into_packet<'b, M: AsyncMiddleware<'b>>(&self, ctx: &mut C, next: &mut M) -> impl Future<Output = Result<(), Error>> {
        next.poll_into_payload(self, ctx)
    }

    #[inline(always)]
    fn poll_from_packet<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> impl Future<Output = Result<Self, Error>> {
        next.poll_from_payload(ctx)
    }
}

pub trait AnyBox<'a>: Send + Sync + 'a {}
impl<'a, T: Send + Sync + 'a> AnyBox<'a> for T {}

/// The `PayloadInfo` trait provides metadata about the payload.
///
/// ### Associated Constants
/// - `const HASH: u64`: A constant hash value associated with the type.
/// - `const TYPE: &'static str`: A string representing the type of the payload.
/// - `const SIZE: Option<usize>`: An optional constant representing the size of the payload.
pub trait PayloadInfo {
    const HASH: u64 = PayloadConstHash(Self::TYPE.as_bytes());
    const TYPE: &'static str = "Unknown";
    const SIZE: Option<usize> = None;
}

pub mod middleware;
pub mod error;
pub mod info;
pub mod features;

#[cfg(feature = "crossbeam")]
pub mod stack;

#[cfg(feature = "crossbeam")]
pub use stack::*;

#[cfg(feature = "sync")]
pub mod payload;

#[cfg(feature = "async")]
pub mod poll_payload;

#[cfg(feature = "async")]
use core::future::Future;

#[doc(hidden)]
pub use xxhash_rust::const_xxh3::xxh3_64 as PayloadConstHash;
#[doc(hidden)]
pub use xxhash_rust::xxh3::xxh3_64 as PayloadHash;

pub use error::*;
pub use middleware::*;
pub use npsd_schema::*;