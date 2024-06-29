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
//! use npsd::{Payload, Schema};
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
//!     // Create an instance of `Animal`.
//!     let animal = Animal::Frog("Frog".to_string(), vec![12393818, -19383812, 11111, -1093838482]);
//!
//!     // Serialize the `animal` instance into a packet.
//!     let serialized = animal.into_packet(&mut (), 1470).unwrap();
//!
//!     // Deserialize the packet back into an `Animal` instance.
//!     let deserialized = Animal::from_packet(&mut (), serialized).unwrap();
//!
//!     // Ensure the deserialized instance matches the original.
//!     assert_eq!(deserialized, animal);
//! }
//! ```

pub type PayloadHandler<'a> = ::std::borrow::Cow<'a, [u8]>;

/// The `PayloadContext` trait provides a way to unwrap the context used in the payload processing.
///
/// # Associated Types
/// - `Context`: The type of the context being unwrapped.
///
/// # Methods
/// - `fn unwrap(&mut self) -> &Self::Context`:
///     - Unwraps the context, returning a reference to it.
pub trait PayloadContext {
    type Context: ?Sized;

    fn unwrap(&mut self) -> &Self::Context;
}

/// The `Middleware` trait defines methods for converting types to and from payloads of bytes.
///
/// # Methods
/// - `fn into_payload<C: PayloadContext, T: IntoPayload<C> + PayloadInfo>(
///       &mut self, 
///       value: &T, 
///       handler: &mut PayloadHandler<'_>, 
///       ctx: &mut C
///   ) -> Result<(), Error>`:
///     - Converts a value into a payload of bytes. This method takes a value and a context, 
///       serializes the value into a byte stream, and writes the resulting bytes into the handler.
/// - `fn from_payload<'a, 'b, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo>(
///       &mut self, 
///       handler: &'b mut PayloadHandler<'a>, 
///       ctx: &mut C
///   ) -> Result<T, Error>`:
///     - Converts a payload of bytes back into a value. This method reads bytes from the handler,
///       uses the context to interpret them, and reconstructs the original value.
/// - `fn write<T>(
///       &mut self, 
///       handler: &mut PayloadHandler<'_>, 
///       data: &[T]
///   ) -> Result<(), Error>`:
///     - Writes raw data into the handler. This method takes a slice of data and appends it to the handler
///       after ensuring that the size of the data elements is 1 byte.
/// - `fn read<'a, 'b, T>(
///       &'b mut self, 
///       handler: &'b mut PayloadHandler<'a>, 
///       nbytes: usize
///   ) -> Result<&'a [T], Error>`:
///     - Reads raw data from the handler. This method reads a specified number of bytes from the handler,
///       splits the handler's data accordingly, and returns a slice of the read data.
pub trait Middleware {
    fn into_payload<C: PayloadContext, T: IntoPayload<C> + PayloadInfo>(&mut self, value: &T, handler: &mut PayloadHandler<'_>, ctx: &mut C) -> Result<(), Error>;
            
    fn from_payload<'a, 'b, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo>(&mut self, handler: &'b mut PayloadHandler<'a>, ctx: &mut C) -> Result<T, Error>
        where
            'a: 'b;

    fn write<T>(&mut self, handler: &mut PayloadHandler<'_>, data: &[T]) -> Result<(), Error> {
        debug_assert_eq!(::std::mem::size_of::<T>(), 1, "Size of T must be 1 byte");
    
        let slice = unsafe {
            ::std::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * ::std::mem::size_of::<T>(),
            )
        };
        
        handler.to_mut().extend_from_slice(slice);
    
        Ok(())
    }

    fn read<'a, 'b, T>(&'b mut self, handler: &'b mut PayloadHandler<'a>, nbytes: usize) -> Result<&'a [T], Error> {
        debug_assert_eq!(::std::mem::size_of::<T>(), 1, "Size of T must be 1 byte");

        if handler.len() < nbytes {
            return Err(Error::InvalidLength { expected: nbytes, found: handler.len() });
        }
    
        let slice = match handler {
            ::std::borrow::Cow::Borrowed(slice) => {
                let (left, right) = slice.split_at(nbytes);
                *handler = ::std::borrow::Cow::Borrowed(right);
    
                left
            },
            ::std::borrow::Cow::Owned(vec) => {
                let right = vec.split_off(nbytes);
                let left = ::std::mem::replace(vec, right);
    
                left.leak()
            }
        };

        let len = slice.len() / ::std::mem::size_of::<T>();
    
        Ok(unsafe {
            ::std::slice::from_raw_parts(slice.as_ptr() as *const T, len)
        })
    }
            
}

/// The `IntoPayload` trait is used to convert a type into a payload of bytes.
///
/// # Methods
/// - `fn into_payload<M: Middleware>(
///       &self, 
///       handler: &mut PayloadHandler<'_>, 
///       ctx: &mut C, 
///       next: &mut M
///   ) -> Result<(), Error>`:
///     - Converts a value into a payload of bytes. This method takes the value, context, and middleware, 
///       serializes the value into a byte stream, and writes it into the handler.
pub trait IntoPayload<C: PayloadContext> {
    fn into_payload<M: Middleware>(&self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error>
        where 
            Self: IntoPayload<C>;
}

/// The `FromPayload` trait is used to convert a payload of bytes back into a type.
/// 
/// # Methods
/// - `fn from_payload<'b, M: Middleware>(
///       handler: &'b mut PayloadHandler<'a>, 
///       ctx: &mut C, 
///       next: &'b mut M
///   ) -> Result<Self, Error>`:
///     - Converts a payload of bytes back into a value. This method reads bytes from the handler,
///       uses the context and middleware to interpret them, and reconstructs the original value.
pub trait FromPayload<'a, C: PayloadContext>: Sized {
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where
            Self: FromPayload<'a, C>,
            'a: 'b;
}

/// The `PayloadInfo` trait provides metadata about the payload.
///
/// # Associated Constants
/// - `const HASH: u64`: A constant hash value associated with the type.
/// - `const TYPE: &'static str`: A string representing the type of the payload.
/// - `const SIZE: Option<usize>`: An optional constant representing the size of the payload.
pub trait PayloadInfo {
    const HASH: u64 = PayloadConstHash(Self::TYPE.as_bytes());
    const TYPE: &'static str;
    const SIZE: Option<usize> = None;
}

/// The `Payload` trait combines `IntoPayload` and `FromPayload` to facilitate
/// complete serialization and deserialization of types.
///
/// # Methods
/// - `fn from_packet<'b, B>(
///       ctx: &mut C, 
///       buffer: B
///   ) -> Result<Self, Error>`:
///     - Deserializes a buffer into a value. This method takes a context and a buffer containing the serialized
///       data, and returns the deserialized value.
/// - `fn from_packet_mw<'b, 'c, B, M: Middleware>(
///       ctx: &mut C, 
///       buffer: B, 
///       next: &'c mut M
///   ) -> Result<Self, Error>`:
///     - Deserializes a buffer into a value using a middleware. This method works similarly to `from_packet`, 
///       but additionally uses middleware for the deserialization process.
/// - `fn into_packet<'b>(
///       &'b self, 
///       ctx: &mut C, 
///       capacity: usize
///   ) -> Result<Vec<u8>, Error>`:
///     - Serializes a value into a buffer. This method takes the value, context, and an initial buffer capacity, 
///       serializes the value, and returns the resulting byte buffer.
/// - `fn into_packet_mw<'b, 'c, M: Middleware>(
///       &'b self, 
///       ctx: &mut C, 
///       capacity: usize, 
///       next: &mut M
///   ) -> Result<Vec<u8>, Error>`:
///     - Serializes a value into a buffer using a middleware. This method works similarly to `into_packet`, 
///       but additionally uses middleware for the serialization process.
pub trait Payload<'a, C: PayloadContext>: IntoPayload<C> + FromPayload<'a, C> + PayloadInfo {
    fn from_packet<'b, B>(ctx: &mut C, buffer: B) -> Result<Self, Error>
        where
            Self: FromPayload<'a, C>,
            B: Into<Cow<'a, [u8]>>,
            'b: 'a,
    {
        #[cfg(feature = "tracing")]
        let next = &mut {
            let mut default = TraceInfo::default();

            default.ctx = Self::TYPE;
            default
        };

        #[cfg(not(feature = "tracing"))]
        let next = &mut ();

        <Self>::from_packet_mw(ctx, buffer, next)
    }

    #[inline(always)]
    fn from_packet_mw<'b, 'c, B, M: Middleware>(ctx: &mut C, buffer: B, next: &'c mut M) -> Result<Self, Error>
        where
            Self: FromPayload<'a, C>,
            B: Into<Cow<'a, [u8]>>,
            'b: 'a,
    {
        <Self>::from_payload(&mut buffer.into(), ctx, next)
    }

    fn into_packet<'b>(&'b self, ctx: &mut C, capacity: usize) -> Result<Vec<u8>, Error>
        where
            Self: IntoPayload<C>,
            'b: 'a,
    {
        #[cfg(feature = "tracing")]
        let next = &mut {
            let mut default = TraceInfo::default();

            default.ctx = Self::TYPE;
            default
        };

        #[cfg(not(feature = "tracing"))]
        let next = &mut ();

        self.into_packet_mw(ctx, capacity, next)
    }

    fn into_packet_mw<'b, 'c, M: Middleware>(&'b self, ctx: &mut C, capacity: usize, next: &mut M) -> Result<Vec<u8>, Error>
        where
            Self: IntoPayload<C>,
            'b: 'a,
    {
        let mut buffer = ::std::borrow::Cow::from(Vec::with_capacity(capacity));

        self.into_payload(&mut buffer, ctx, next)?;

        Ok(buffer.to_vec())
    }
}

pub mod middleware;
pub mod error;
pub mod payload;
pub mod features;
pub mod context;

#[doc(hidden)]
pub use xxhash_rust::const_xxh3::xxh3_64 as PayloadConstHash;
#[doc(hidden)]
pub use xxhash_rust::xxh3::xxh3_64 as PayloadHash;
#[doc(hidden)]
use std::borrow::Cow;

#[cfg(feature = "tracing")]
use middleware::tracing::TraceInfo;

pub use error::Error;
pub use npsd_schema::{Schema, Bitmap};