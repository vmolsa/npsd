use std::borrow::Cow;

#[cfg(feature = "info")]
use std::collections::LinkedList;

#[cfg(feature = "sync")]
use crate::{FromPayload, IntoPayload, Middleware};

#[cfg(feature = "async")]
use crate::{AsyncFromPayload, AsyncIntoPayload, AsyncMiddleware};

use crate::Error;

/// A no-op implementation of the `Middleware` and `AsyncMiddleware` traits.
///
/// This implementation is used when no middleware processing is required. It simply forwards the
/// serialization and deserialization tasks to the respective methods of the value being processed,
/// and provides basic read/write operations to the underlying `Cow` buffer.
///
/// # Middleware Methods
/// - `fn into_payload<C, T: IntoPayload<C>>(
///       &mut self,
///       value: &T,
///       ctx: &mut C
///   ) -> Result<(), Error>`:
///     - Converts a value into a payload of bytes. This method forwards the serialization task to
///       the `into_payload` method of the value being processed.
/// - `fn from_payload<'a, 'b, C, T: FromPayload<'a, C>>(
///       &mut self,
///       ctx: &mut C
///   ) -> Result<T, Error>`:
///     - Converts a payload of bytes back into a value. This method forwards the deserialization task
///       to the `from_payload` method of the type being processed.
/// - `fn write<T>(&mut self, data: &[T]) -> Result<(), Error>`:
///     - Writes data to the underlying buffer.
/// - `fn read<'a, 'b, T>(&'b mut self, nbytes: usize) -> Result<&'a [T], Error>`:
///     - Reads data from the underlying buffer.
/// - `fn serialized(&self) -> Vec<u8>`:
///     - Returns a vector containing the serialized data from the underlying buffer.
#[cfg(feature = "sync")]
impl Middleware for Next<'_> {
    #[inline(always)]
    fn into_payload<C, T: IntoPayload<C>>(&mut self, value: &T, ctx: &mut C) -> Result<(), Error> {   
        value.into_payload(ctx, self)
    }

    #[inline(always)]
    fn from_payload<'a, 'b, C, T: FromPayload<'a, C>>(&mut self, ctx: &mut C) -> Result<T, Error>
        where
            'a: 'b 
    {
        T::from_payload(ctx, self)
    }

    #[inline(always)]
    fn write<T>(&mut self, data: &[T]) -> Result<(), Error> {
        self.0.write(data)
    }

    #[inline(always)]
    fn read<'a, 'b, T>(&'b mut self, nbytes: usize) -> Result<&'a [T], Error> {
        self.0.read(nbytes)
    }
}

/// # AsyncMiddleware Methods
/// - `async fn poll_into_payload<C, T: AsyncIntoPayload<C>>(
///       &mut self,
///       value: &T,
///       ctx: &mut C
///   ) -> Result<(), Error>`:
///     - Asynchronously converts a value into a payload of bytes. This method forwards the serialization
///       task to the `poll_into_payload` method of the value being processed.
/// - `async fn poll_from_payload<'a, C, T: AsyncFromPayload<'a, C>>(
///       &mut self,
///       ctx: &mut C
///   ) -> Result<T, Error>`:
///     - Asynchronously converts a payload of bytes back into a value. This method forwards the deserialization
///       task to the `poll_from_payload` method of the type being processed.
/// - `async fn poll_write<T>(&mut self, data: &[T]) -> Result<(), Error>`:
///     - Asynchronously writes data to the underlying buffer.
/// - `async fn poll_read<'a, 'b, T: 'a>(&'b mut self, nbytes: usize) -> Result<&'a [T], Error>`:
///     - Asynchronously reads data from the underlying buffer.
#[cfg(feature = "async")]
impl AsyncMiddleware for Next<'_> {
    #[inline(always)]
    fn poll_into_payload<C: Send + Sync, T: AsyncIntoPayload<C>>(
        &mut self,
        value: &T,
        ctx: &mut C
    ) -> impl core::future::Future<Output = Result<(), Error>> {
        value.poll_into_payload(ctx, self)
    }
    
    #[inline(always)]
    fn poll_from_payload<'a, 'b, C: Send + Sync, T: AsyncFromPayload<'a, C>> (
        &'b mut self,
        ctx: &mut C,
    ) -> impl core::future::Future<Output = Result<T, Error>>
        where 'a: 'b
    {
        T::poll_from_payload(ctx, self)
    }

    #[inline(always)]
    async fn poll_write<T>(&mut self, data: &[T]) -> Result<(), Error> {
        self.0.write(data)
    }

    #[inline(always)]
    async fn poll_read<'a, 'b, T: 'a>(&mut self, nbytes: usize) -> Result<&'a [T], Error> {
        self.0.read(nbytes)
    }
}

/// # Trait Implementations
/// ## CowRw
/// Provides basic read and write operations for `Cow<'_, [u8]>`:
/// - `fn write<T>(&mut self, data: &[T]) -> Result<(), Error>`:
///     - Writes data to the underlying `Cow` buffer, ensuring the size of `T` is 1 byte.
/// - `fn read<'a, 'b, T>(&'b mut self, nbytes: usize) -> Result<&'a [T], Error>`:
///     - Reads data from the underlying `Cow` buffer, ensuring the size of `T` is 1 byte.
pub trait CowRw {
    fn write<T>(&mut self, data: &[T]) -> Result<(), Error>;
    fn read<'a, 'b, T>(&'b mut self, nbytes: usize) -> Result<&'a [T], Error>;
}

impl CowRw for Cow<'_, [u8]> {
    fn write<T>(&mut self, data: &[T]) -> Result<(), Error> {
        debug_assert_eq!(::std::mem::size_of::<T>(), 1, "Size of T must be 1 byte");
    
        let slice = unsafe {
            ::std::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * ::std::mem::size_of::<T>(),
            )
        };
        
        self.to_mut().extend_from_slice(slice);
    
        Ok(())
    }

    fn read<'a, 'b, T>(&'b mut self, nbytes: usize) -> Result<&'a [T], Error> {
        debug_assert_eq!(::std::mem::size_of::<T>(), 1, "Size of T must be 1 byte");

        if self.len() < nbytes {
            return Err(Error::InvalidLength { expected: nbytes, found: self.len() });
        }
    
        let slice = match self {
            ::std::borrow::Cow::Borrowed(slice) => {
                let (left, right) = slice.split_at(nbytes);
                *self = ::std::borrow::Cow::Borrowed(right);
    
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

/// # Structs
/// ## Next<'a>
/// A wrapper around a `Cow<'a, [u8]>` buffer, providing methods for creating and managing the buffer:
/// - `pub fn with_mtu(mtu: usize) -> Self`:
///     - Creates a new `Next` instance with a buffer capacity specified by `mtu`.
/// - `impl<'a> Default for Next<'a>`:
///     - Provides a default implementation that creates a `Next` instance with an empty buffer.
#[derive(Clone, Debug)]
pub struct Next<'a>(Cow<'a, [u8]>);

impl<'a> Next<'a> {
    pub fn with_mtu(mtu: usize) -> Self {
        Self(Cow::from(Vec::with_capacity(mtu)))
    }

    #[inline(always)]
    pub fn serialized(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl<'a> Default for Next<'a> {
    fn default() -> Self {
        Next(Cow::from(Vec::new()))
    }
}

impl<'a, T: Into<Cow<'a, [u8]>>> From<T> for Next<'a> {
    fn from(value: T) -> Self {
        Next(value.into())
    }
}

/// The `NextTrace` struct provides middleware functionality for tracing and debugging serialization
/// and deserialization operations. It maintains a nested context to help identify and report errors
/// and their locations in the payload structure. Requires the `info` feature to be enabled.
#[cfg(feature = "info")]
#[derive(Clone, Debug)]
pub struct NextTrace<'a>(Cow<'a, [u8]>, usize, LinkedList<&'static str>);

#[cfg(feature = "info")]
const MAX_NESTED_DEPTH: usize = 255;

#[cfg(feature = "info")]
impl<'a> NextTrace<'a> {
    pub fn with_mtu(mtu: usize) -> Self {
        Self(Cow::from(Vec::with_capacity(mtu)), MAX_NESTED_DEPTH, LinkedList::new())
    }

    pub fn with_depth(depth: usize) -> Self {
        Self(Cow::from(Vec::new()), depth, LinkedList::new())
    }

    #[inline(always)]
    pub fn serialized(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

#[cfg(feature = "info")]
impl<'a, T: Into<Cow<'a, [u8]>>> From<T> for NextTrace<'a> {
    fn from(value: T) -> Self {
        Self(value.into(), MAX_NESTED_DEPTH, LinkedList::new())
    }
}

#[cfg(feature = "info")]
impl<'a> Default for NextTrace<'a> {
    fn default() -> Self {
        NextTrace(Cow::from(Vec::new()), MAX_NESTED_DEPTH, LinkedList::new())
    }
}

#[cfg(feature = "info")]
#[cfg(feature = "sync")]
impl Middleware for NextTrace<'_> {
    #[inline(always)]
    fn into_payload<C, T: IntoPayload<C>>(&mut self, value: &T, ctx: &mut C) -> Result<(), Error> {
        if self.2.len() > self.1 {
            return Err(Error::NestedDepthLimit(T::TYPE.to_string()))
        }

        self.2.push_back(T::TYPE);
 
        let path: Vec<&str> = self.2.iter().cloned().collect();
        println!("{}", path.join(" -> "));

        match value.into_payload(ctx, self) {
            Ok(value) => {
                self.2.pop_back();

                Ok(value)
            },
            Err(error) => {
                match error {
                    Error::Traced(_, _) => Err(error),
                    _ => {
                        let path: Vec<&str> = self.2.iter().cloned().collect();
                        Err(Error::Traced(error.to_string(), path.join(" -> ")))
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn from_payload<'a, 'b, C, T: FromPayload<'a, C>>(&mut self, ctx: &mut C) -> Result<T, Error>
        where
            'a: 'b 
    {
        if self.2.len() > self.1 {
            return Err(Error::NestedDepthLimit(T::TYPE.to_string()))
        }

        self.2.push_back(T::TYPE);

        let path: Vec<&str> = self.2.iter().cloned().collect();
        println!("{}", path.join(" <- "));

        match T::from_payload(ctx, self) {
            Ok(value) => {
                self.2.pop_back();

                Ok(value)
            },
            Err(error) => {
                match error {
                    Error::Traced(_, _) => Err(error),
                    _ => {
                        let path: Vec<&str> = self.2.iter().cloned().collect();
                        Err(Error::Traced(error.to_string(), path.join(" <- ")))
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn write<T>(&mut self, data: &[T]) -> Result<(), Error> {
        self.0.write(data)
    }

    #[inline(always)]
    fn read<'a, 'b, T>(&'b mut self, nbytes: usize) -> Result<&'a [T], Error> {
        self.0.read(nbytes)
    }
}

#[cfg(feature = "info")]
#[cfg(feature = "async")]
impl AsyncMiddleware for NextTrace<'_> {
    #[inline(always)]
    async fn poll_into_payload<C: Send + Sync, T: AsyncIntoPayload<C>>(
        &mut self,
        value: &T,
        ctx: &mut C
    ) -> Result<(), Error> {
        if self.2.len() > self.1 {
            return Err(Error::NestedDepthLimit(T::TYPE.to_string()));
        }

        self.2.push_back(T::TYPE);

        let path: Vec<&str> = self.2.iter().cloned().collect();
        println!("{}", path.join(" -> "));

        match value.poll_into_payload(ctx, self).await {
            Ok(value) => {
                self.2.pop_back();
                Ok(value)
            }
            Err(error) => {
                match error {
                    Error::Traced(_, _) => Err(error),
                    _ => {
                        let path: Vec<&str> = self.2.iter().cloned().collect();
                        Err(Error::Traced(error.to_string(), path.join(" -> ")))
                    }
                }
            }
        }
    }
    
    #[inline(always)]
    async fn poll_from_payload<'a, 'b, C: Send + Sync, T: AsyncFromPayload<'a, C>> (
        &'b mut self, 
        ctx: &mut C,
    ) -> Result<T, Error> 
        where 'a: 'b,
    {
        if self.2.len() > self.1 {
            return Err(Error::NestedDepthLimit(T::TYPE.to_string()));
        }

        self.2.push_back(T::TYPE);

        let path: Vec<&str> = self.2.iter().cloned().collect();
        println!("{}", path.join(" <- "));

        match T::poll_from_payload(ctx, self).await {
            Ok(value) => {
                self.2.pop_back();
                Ok(value)
            }
            Err(error) => {
                match error {
                    Error::Traced(_, _) => Err(error),
                    _ => {
                        let path: Vec<&str> = self.2.iter().cloned().collect();
                        Err(Error::Traced(error.to_string(), path.join(" <- ")))
                    }
                }
            }
        }
    }

    #[inline(always)]
    async fn poll_write<T>(&mut self, data: &[T]) -> Result<(), Error> {
        self.0.write(data)
    }

    #[inline(always)]
    async fn poll_read<'a, 'b, T: 'a>(&mut self, nbytes: usize) -> Result<&'a [T], Error> {
        self.0.read(nbytes)
    }
}