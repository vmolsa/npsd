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
//! ## Example
//! ```rust
//! use npsd::{Payload, Schema, Next, Middleware};
//!
//! #[derive(Schema, PartialEq, Debug)]
//! enum Animal {
//!     Dog,
//!     Frog(String, Vec<isize>),
//!     Cat { age: usize, name: String },
//!     AntHive(Vec<String>),
//! }
//!
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
//!     // Create copy of serialized data if needed
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
//! ## Middleware Trait
//!
//! The `Middleware` trait defines methods for converting types to and from payloads of bytes.
//!
//! ### Methods
//!
//! - `fn into_payload<C, T: IntoPayload<C> + PayloadInfo>(&mut self, value: &T, ctx: &mut C) -> Result<(), Error>`:
//!     - Converts a value into a payload of bytes. This method takes a value and a context, serializes the value into a byte stream, and writes the resulting bytes into the handler.
//! - `fn from_payload<'a, 'b, C, T: FromPayload<'a, C> + PayloadInfo>(&mut self, ctx: &mut C) -> Result<T, Error>`:
//!     - Converts a payload of bytes back into a value. This method reads bytes from the handler, uses the context to interpret them, and reconstructs the original value.
//! - `fn write<T>(&mut self, data: &[T]) -> Result<(), Error>`:
//!     - Writes raw data into the handler. This method takes a slice of data and appends it to the handler after ensuring that the size of the data elements is 1 byte.
//! - `fn read<'a, 'b, T>(&'b mut self, nbytes: usize) -> Result<&'a [T], Error>`:
//!     - Reads raw data from the handler. This method reads a specified number of bytes from the handler, splits the handler's data accordingly, and returns a slice of the read data.
//! - `fn serialized(&self) -> Vec<u8>`:
//!     - Returns the serialized data as a `Vec<u8>`.
pub trait Middleware {
    fn into_payload<C, T: IntoPayload<C> + PayloadInfo>(&mut self, value: &T, ctx: &mut C) -> Result<(), Error>;
    fn from_payload<'a, 'b, C, T: FromPayload<'a, C> + PayloadInfo>(&mut self, ctx: &mut C) -> Result<T, Error>
        where 'a: 'b;
    fn write<T>(&mut self, data: &[T]) -> Result<(), Error>;
    fn read<'a, 'b, T>(&'b mut self, nbytes: usize) -> Result<&'a [T], Error>;
    fn serialized(&self) -> Vec<u8>;
}

/// The `AsyncMiddleware` trait extends `Middleware` with asynchronous methods for converting types to and from payloads of bytes.
///
/// ### Methods
/// - `fn poll_into_payload<'a, C, T: AsyncIntoPayload<C>>(&mut self, value: &T, ctx: &mut C) -> impl Future<Output = Result<(), Error>>`:
///     - Polls the conversion of a value into a payload of bytes asynchronously.
/// - `fn poll_from_payload<'a, C, T: AsyncFromPayload<'a, C>>(&mut self, ctx: &mut C) -> impl Future<Output = Result<T, Error>>`:
///     - Polls the conversion of a payload of bytes back into a value asynchronously.
/// - `fn poll_write<T>(&mut self, data: &[T]) -> impl Future<Output = Result<(), Error>>`:
///     - Polls the asynchronous writing of raw data into the handler.
/// - `fn poll_read<'a, 'b, T: 'a>(&'b mut self, nbytes: usize) -> impl Future<Output = Result<&'a [T], Error>>`:
///     - Polls the asynchronous reading of raw data from the handler.
pub trait AsyncMiddleware: Middleware {
    fn poll_into_payload<'a, C, T: AsyncIntoPayload<C>>(&mut self, value: &T, ctx: &mut C) -> impl Future<Output = Result<(), Error>>;
    fn poll_from_payload<'a, C, T: AsyncFromPayload<'a, C>>(&mut self, ctx: &mut C) -> impl Future<Output = Result<T, Error>>;
    fn poll_write<T>(&mut self, data: &[T]) -> impl Future<Output = Result<(), Error>>;
    fn poll_read<'a, 'b, T: 'a>(&'b mut self, nbytes: usize) -> impl Future<Output = Result<&'a [T], Error>>;
}

/// The `IntoPayload` trait is used to convert a type into a payload of bytes.
///
/// ### Methods
/// - `fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error>`:
///     - Converts a value into a payload of bytes. This method takes the value, context, and middleware, serializes the value into a byte stream, and writes it into the handler.
pub trait IntoPayload<C> {
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error>;
}

/// The `AsyncIntoPayload` trait extends `IntoPayload` with asynchronous methods for converting types into payloads of bytes.
///
/// ### Methods
/// - `fn poll_into_payload<M: AsyncMiddleware>(&self, ctx: &mut C, next: &mut M) -> impl Future<Output = Result<(), Error>>`:
///     - Polls the conversion of a value into a payload of bytes asynchronously.
pub trait AsyncIntoPayload<C>: IntoPayload<C> {
    fn poll_into_payload<M: AsyncMiddleware>(&self, ctx: &mut C, next: &mut M) -> impl Future<Output = Result<(), Error>>;
}

/// The `FromPayload` trait is used to convert a payload of bytes back into a type.
/// 
/// ### Methods
/// - `fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>`:
///     - Converts a payload of bytes back into a value. This method reads bytes from the handler, uses the context and middleware to interpret them, and reconstructs the original value.
pub trait FromPayload<'a, C>: Sized {
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b;
}

/// The `AsyncFromPayload` trait extends `FromPayload` with asynchronous methods for converting payloads of bytes back into types.
///
/// ### Methods
/// - `fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> impl Future<Output = Result<Self, Error>>`:
///     - Polls the conversion of a payload of bytes back into a value asynchronously.
pub trait AsyncFromPayload<'a, C>: FromPayload<'a, C> {
    fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> impl Future<Output = Result<Self, Error>>
        where 'a: 'b;
}

/// The `PayloadInfo` trait provides metadata about the payload.
///
/// ### Associated Constants
/// - `const HASH: u64`: A constant hash value associated with the type.
/// - `const TYPE: &'static str`: A string representing the type of the payload.
/// - `const SIZE: Option<usize>`: An optional constant representing the size of the payload.
pub trait PayloadInfo {
    const HASH: u64 = PayloadConstHash(Self::TYPE.as_bytes());
    const TYPE: &'static str;
    const SIZE: Option<usize> = None;
}

/// The `Payload` trait combines `IntoPayload` and `FromPayload` to facilitate complete serialization and deserialization of types.
///
/// ### Methods
/// - `fn from_packet<'b, 'c, M: Middleware>(ctx: &mut C, next: &'c mut M) -> Result<Self, Error>`:
///     - Deserializes a buffer into a value. This method takes a context and a buffer containing the serialized data, and returns the deserialized value.
/// - `fn into_packet<'b, 'c, M: Middleware>(&'b self, ctx: &mut C, next: &mut M) -> Result<(), Error>`:
///     - Serializes a value into a buffer. This method takes the value, context, and an initial buffer capacity, serializes the value, and returns the resulting byte buffer.
pub trait Payload<'a, C>: IntoPayload<C> + FromPayload<'a, C> + PayloadInfo {
    #[inline(always)]
    fn from_packet<'b, 'c, M: Middleware>(ctx: &mut C, next: &'c mut M) -> Result<Self, Error>
        where Self: FromPayload<'a, C>, 'b: 'a,
    {
        <Self>::from_payload(ctx, next)
    }

    fn into_packet<'b, 'c, M: Middleware>(&'b self, ctx: &mut C, next: &mut M) -> Result<(), Error>
        where Self: IntoPayload<C>, 'b: 'a,
    {
        self.into_payload(ctx, next)
    }
}

/// The `AsyncPayload` trait extends `PayloadInfo` with asynchronous methods for complete serialization and deserialization of types.
///
/// ### Methods
/// - `fn poll_into_packet<'b, M: AsyncMiddleware>(&'b self, ctx: &'b mut C, next: &'b mut M) -> impl Future<Output = Result<(), Error>>`:
///     - Initiates the asynchronous conversion of a value into a packet.
/// - `fn poll_from_packet<'b, M: AsyncMiddleware>(ctx: &'b mut C, next: &'b mut M) -> impl Future<Output = Result<Self, Error>>`:
///     - Initiates the asynchronous deserialization of a packet into a value.
pub trait AsyncPayload<C>: PayloadInfo {
    fn poll_into_packet<'b, M: AsyncMiddleware>(&'b self, ctx: &'b mut C, next: &'b mut M) -> impl Future<Output = Result<(), Error>>
        where Self: AsyncIntoPayload<C> + Sized
    {
        next.poll_into_payload(self, ctx)
    }

    fn poll_from_packet<'b, M: AsyncMiddleware>(ctx: &'b mut C, next: &'b mut M) -> impl Future<Output = Result<Self, Error>>
        where Self: AsyncFromPayload<'b, C> + Sized
    {
       next.poll_from_payload(ctx)
    }
}

pub mod middleware;
pub mod error;
pub mod payload;
pub mod poll_payload;
pub mod features;

use core::future::Future;

#[doc(hidden)]
pub use xxhash_rust::const_xxh3::xxh3_64 as PayloadConstHash;
#[doc(hidden)]
pub use xxhash_rust::xxh3::xxh3_64 as PayloadHash;

#[cfg(feature = "tracing")]
use middleware::tracing::TraceInfo;

pub use error::*;
pub use poll_payload::*;
pub use middleware::*;
pub use npsd_schema::*;
