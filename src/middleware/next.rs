use std::borrow::Cow;

#[cfg(feature = "info")]
use std::collections::LinkedList;

#[cfg(feature = "sync")]
use crate::{FromPayload, IntoPayload, Middleware};

#[cfg(feature = "async")]
use crate::{AsyncFromPayload, AsyncIntoPayload, AsyncMiddleware};

#[cfg(feature = "crossbeam")]
use crate::Stack;

use crate::{AnyBox, Error};

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
/// - `fn from_payload<'a, C, T: FromPayload<'a, C>>(
///       &mut self,
///       ctx: &mut C
///   ) -> Result<T, Error>`:
///     - Converts a payload of bytes back into a value. This method forwards the deserialization task
///       to the `from_payload` method of the type being processed.
/// - `fn write<T>(&mut self, data: &[T]) -> Result<(), Error>`:
///     - Writes data to the underlying buffer.
/// - `fn read<'a, T>(&mut self, nbytes: usize) -> Result<&'a [T], Error>`:
///     - Reads data from the underlying buffer.
/// - `fn read_mut<'a, T>(&mut self, nbytes: usize) -> Result<&'a mut [T], Error>`:
///     - Reads data from the underlying buffer as mut.
/// - `fn push<T: AnyBox<'a>>(&mut self, value: Box<T>) -> Result<&'a T, Error>`:
///     - Pushes a value onto the stack.
/// - `fn push_mut<T: AnyBox<'a>>(&mut self, value: Box<T>) -> Result<&'a mut T, Error>`:
///     - Pushes a mutable value onto the stack.
/// - `fn push_array<T: AnyBox<'a>>(&mut self, values: Box<[T]>) -> Result<&'a [T], Error>`:
///     - Pushes an array of values onto the stack.
/// - `fn push_array_mut<T: AnyBox<'a>>(&mut self, values: Box<[T]>) -> Result<&'a mut [T], Error>`:
///     - Pushes a mutable array of values onto the stack.
#[cfg(feature = "sync")]
impl<'a> Middleware<'a> for Next<'a> {
    #[inline(always)]
    fn into_payload<C, T: IntoPayload<C>>(&mut self, value: &T, ctx: &mut C) -> Result<(), Error> {   
        value.into_payload(ctx, self)
    }

    #[inline(always)]
    fn from_payload<C, T: FromPayload<'a, C>>(&mut self, ctx: &mut C) -> Result<T, Error> {
        T::from_payload(ctx, self)
    }

    #[inline(always)]
    fn write<T>(&mut self, data: &[T]) -> Result<(), Error> {
        self.buf.write(data)
    }

    #[inline(always)]
    fn read<T>(&mut self, nbytes: usize) -> Result<&'a [T], Error> {
        self.buf.read(nbytes)
    }

    #[inline(always)]
    fn read_mut<T>(&mut self, nbytes: usize) -> Result<&'a mut [T], Error> {
        self.buf.read_mut(nbytes)
    }

    #[cfg(feature = "crossbeam")]
    #[inline(always)]
    fn push<T: AnyBox<'a>>(&mut self, value: Box<T>) -> Result<&'a T, Error> {
        Ok(self.stack.push(value))
    }

    #[cfg(feature = "crossbeam")]
    #[inline(always)]
    fn push_mut<T: AnyBox<'a>>(&mut self, value: Box<T>) -> Result<&'a mut T, Error> {
        Ok(self.stack.push_mut(value))
    }

    #[cfg(feature = "crossbeam")]
    #[inline(always)]
    fn push_array<T: AnyBox<'a>>(&mut self, values: Box<[T]>) -> Result<&'a [T], Error> {
        Ok(self.stack.push_array(values))
    }

    #[cfg(feature = "crossbeam")]
    #[inline(always)]
    fn push_array_mut<T: AnyBox<'a>>(&mut self, values: Box<[T]>) -> Result<&'a mut [T], Error> {
        Ok(self.stack.push_array_mut(values))
    }

    #[cfg(not(feature = "crossbeam"))]
    #[inline(always)]
    fn push<T: AnyBox<'a>>(&mut self, _value: Box<T>) -> Result<&'a T, Error> {
        Err(Error::Stack("References disabled".to_string()))
    }

    #[cfg(not(feature = "crossbeam"))]
    #[inline(always)]
    fn push_mut<T: AnyBox<'a>>(&mut self, _value: Box<T>) -> Result<&'a mut T, Error> {
        Err(Error::Stack("References disabled".to_string()))
    }

    #[cfg(not(feature = "crossbeam"))]
    #[inline(always)]
    fn push_array<T: AnyBox<'a>>(&mut self, _values: Box<[T]>) -> Result<&'a [T], Error> {
        Err(Error::Stack("References disabled".to_string()))
    }

    #[cfg(not(feature = "crossbeam"))]
    #[inline(always)]
    fn push_array_mut<T: AnyBox<'a>>(&mut self, _values: Box<[T]>) -> Result<&'a mut [T], Error> {
        Err(Error::Stack("References disabled".to_string()))
    }
}

/// # AsyncMiddleware Methods
/// - `fn poll_into_payload<C, T: AsyncIntoPayload<C>>(
///       &mut self,
///       value: &T,
///       ctx: &mut C
///   ) -> impl Future<Output = Result<(), Error>>`:
///     - Asynchronously converts a value into a payload of bytes. This method forwards the serialization
///       task to the `poll_into_payload` method of the value being processed.
/// - `fn poll_from_payload<'a, C, T: AsyncFromPayload<'a, C>>(
///       &mut self,
///       ctx: &mut C
///   ) -> impl Future<Output = Result<T, Error>>`:
///     - Asynchronously converts a payload of bytes back into a value. This method forwards the deserialization
///       task to the `poll_from_payload` method of the type being processed.
/// - `async fn poll_write<T>(&mut self, data: &[T]) -> Result<(), Error>`:
///     - Asynchronously writes data to the underlying buffer.
/// - `async fn poll_read<T: 'a>(&mut self, nbytes: usize) -> Result<&'a [T], Error>`:
///     - Asynchronously reads data from the underlying buffer.
/// - `async fn poll_read_mut<T: 'a>(&mut self, nbytes: usize) -> Result<&'a mut [T], Error>`:
///     - Asynchronously reads data from the underlying buffer as mut.
/// - `async fn poll_push<T: AnyBox<'a>>(&mut self, value: Box<T>) -> Result<&'a T, Error>`:
///     - Asynchronously pushes a value onto the stack.
/// - `async fn poll_push_mut<T: AnyBox<'a>>(&mut self, value: Box<T>) -> Result<&'a mut T, Error>`:
///     - Asynchronously pushes a mutable value onto the stack.
/// - `async fn poll_push_array<T: AnyBox<'a>>(&mut self, values: Box<[T]>) -> Result<&'a [T], Error>`:
///     - Asynchronously pushes an array of values onto the stack.
/// - `async fn poll_push_array_mut<T: AnyBox<'a>>(&mut self, values: Box<[T]>) -> Result<&'a mut [T], Error>`:
///     - Asynchronously pushes a mutable array of values onto the stack.
#[cfg(feature = "async")]
impl<'a> AsyncMiddleware<'a> for Next<'a> {
    #[inline(always)]
    fn poll_into_payload<C: Send + Sync, T: AsyncIntoPayload<C>>(
        &mut self,
        value: &T,
        ctx: &mut C
    ) -> impl core::future::Future<Output = Result<(), Error>> {
        value.poll_into_payload(ctx, self)
    }
    
    #[inline(always)]
    fn poll_from_payload<C: Send + Sync, T: AsyncFromPayload<'a, C>> (
        &mut self,
        ctx: &mut C,
    ) -> impl core::future::Future<Output = Result<T, Error>> {
        T::poll_from_payload(ctx, self)
    }

    #[inline(always)]
    async fn poll_write<T>(&mut self, data: &[T]) -> Result<(), Error> {
        self.buf.write(data)
    }

    #[inline(always)]
    async fn poll_read<T: 'a>(&mut self, nbytes: usize) -> Result<&'a [T], Error> {
        self.buf.read(nbytes)
    }

    #[inline(always)]
    async fn poll_read_mut<T: 'a>(&mut self, nbytes: usize) -> Result<&'a mut [T], Error> {
        self.buf.read_mut(nbytes)
    }

    #[cfg(feature = "crossbeam")]
    #[inline(always)]
    async fn poll_push<T: AnyBox<'a>>(&mut self, value: Box<T>) -> Result<&'a T, Error> {
        Ok(self.stack.push(value))
    }

    #[cfg(feature = "crossbeam")]
    #[inline(always)]
    async fn poll_push_mut<T: AnyBox<'a>>(&mut self, value: Box<T>) -> Result<&'a mut T, Error> {
        Ok(self.stack.push_mut(value))
    }

    #[cfg(feature = "crossbeam")]
    #[inline(always)]
    async fn poll_push_array<T: AnyBox<'a>>(&mut self, values: Box<[T]>) -> Result<&'a [T], Error> {
        Ok(self.stack.push_array(values))
    }

    #[cfg(feature = "crossbeam")]
    #[inline(always)]
    async fn poll_push_array_mut<T: AnyBox<'a>>(&mut self, values: Box<[T]>) -> Result<&'a mut [T], Error> {
        Ok(self.stack.push_array_mut(values))
    }

    #[cfg(not(feature = "crossbeam"))]
    #[inline(always)]
    async fn poll_push<T: AnyBox<'a>>(&mut self, _value: Box<T>) -> Result<&'a T, Error> {
        Err(Error::Stack("References disabled".to_string()))
    }

    #[cfg(not(feature = "crossbeam"))]
    #[inline(always)]
    async fn poll_push_mut<T: AnyBox<'a>>(&mut self, _value: Box<T>) -> Result<&'a mut T, Error> {
        Err(Error::Stack("References disabled".to_string()))
    }

    #[cfg(not(feature = "crossbeam"))]
    #[inline(always)]
    async fn poll_push_array<T: AnyBox<'a>>(&mut self, _values: Box<[T]>) -> Result<&'a [T], Error> {
        Err(Error::Stack("References disabled".to_string()))
    }

    #[cfg(not(feature = "crossbeam"))]
    #[inline(always)]
    async fn poll_push_array_mut<T: AnyBox<'a>>(&mut self, _values: Box<[T]>) -> Result<&'a mut [T], Error> {
        Err(Error::Stack("References disabled".to_string()))
    }
}

/// # Trait Implementations
/// ## CowRw
/// Provides basic read and write operations for `Cow<'_, [u8]>`:
/// - `fn write<T>(&mut self, data: &[T]) -> Result<(), Error>`:
///     - Writes data to the underlying `Cow` buffer, ensuring the size of `T` is 1 byte.
/// - `fn read<'a, T>(&mut self, nbytes: usize) -> Result<&'a [T], Error>`:
///     - Reads data from the underlying `Cow` buffer, ensuring the size of `T` is 1 byte.
/// - `fn read_mut<'a, T>(&mut self, nbytes: usize) -> Result<&'a mut [T], Error>`:
///     - Reads data as mut from the underlying `Cow` buffer, ensuring the size of `T` is 1 byte.
pub trait CowRw {
    fn write<T>(&mut self, data: &[T]) -> Result<(), Error>;
    fn read<'a, 'b, T>(&'b mut self, nbytes: usize) -> Result<&'a [T], Error>;
    fn read_mut<'a, 'b, T>(&'b mut self, nbytes: usize) -> Result<&'a mut [T], Error>;
}

impl CowRw for (Cow<'_, [u8]>, usize) {
    fn write<T>(&mut self, data: &[T]) -> Result<(), Error> {
        debug_assert_eq!(::std::mem::size_of::<T>(), 1, "Size of T must be 1 byte");
    
        let slice = unsafe {
            ::std::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * ::std::mem::size_of::<T>(),
            )
        };
        
        self.0.to_mut().extend_from_slice(slice);
    
        Ok(())
    }

    fn read<'a, T>(&mut self, nbytes: usize) -> Result<&'a [T], Error> {
        debug_assert_eq!(::std::mem::size_of::<T>(), 1, "Size of T must be 1 byte");

        if self.0.len() < self.1 + nbytes {
            return Err(Error::InvalidLength { expected: nbytes, found: self.0.len() - self.1 });
        }

        let slice = &self.0[self.1..self.1 + nbytes];
        self.1 += nbytes;

        let len = slice.len() / ::std::mem::size_of::<T>();
    
        Ok(unsafe {
            ::std::slice::from_raw_parts(slice.as_ptr() as *const T, len)
        })
    }

    fn read_mut<'a, T>(&mut self, nbytes: usize) -> Result<&'a mut [T], Error> {
        debug_assert_eq!(::std::mem::size_of::<T>(), 1, "Size of T must be 1 byte");

        if self.0.len() < self.1 + nbytes {
            return Err(Error::InvalidLength { expected: nbytes, found: self.0.len() - self.1 });
        }

        let slice = &mut self.0.to_mut()[self.1..self.1 + nbytes];
        self.1 += nbytes;

        let len = slice.len() / ::std::mem::size_of::<T>();
    
        Ok(unsafe {
            ::std::slice::from_raw_parts_mut(slice.as_ptr() as *mut T, len)
        })
    }
}

/// # Structs
/// ## Next<'a>
/// A wrapper around a `Cow<'a, [u8]>` buffer, providing methods for creating and managing the buffer:
/// - `pub fn from_mut(cow: &'a mut Cow<'_, [u8]>) -> Self`:
///     - Creates a new `Next` instance from a mutable reference to a `Cow` buffer.
/// - `pub fn with_mtu(mtu: usize) -> Self`:
///     - Creates a new `Next` instance with a buffer capacity specified by `mtu`.
/// - `impl<'a> Default for Next<'a>`:
///     - Provides a default implementation that creates a `Next` instance with an empty buffer.
/// - `impl<'a, T: Into<Cow<'a, [u8]>>> From<T> for Next<'a>`:
///     - Provides an implementation that creates a `Next` instance from a value that can be converted
///       into a `Cow` buffer.
/// - `pub fn serialized(&self) -> Vec<u8>`:
///     - Returns a vector containing the serialized data from the underlying buffer.
/// - `pub fn as_slice(&self) -> &[u8]`:
///     - Returns a slice of the underlying buffer.
#[derive(Clone, Debug)]
pub struct Next<'a>{
    buf: (Cow<'a, [u8]>, usize),

    #[cfg(feature = "crossbeam")]
    stack: Stack<'a>,
}

impl<'a> Next<'a> {
    pub fn from_mut(cow: &'a mut Cow<'_, [u8]>) -> Self {
        Self {
            buf: (Cow::Borrowed(&*cow), 0),
            #[cfg(feature = "crossbeam")]
            stack: Stack::new(),
        }
    }

    pub fn with_mtu(mtu: usize) -> Self {
        Self {
            buf: (Cow::from(Vec::with_capacity(mtu)), 0),
            #[cfg(feature = "crossbeam")]
            stack: Stack::new(),
        }
    }

    #[inline(always)]
    pub fn serialized(&self) -> Vec<u8> {
        self.buf.0.to_vec()
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[u8] { 
        self.buf.0.as_ref()
    }
}

impl<'a> Default for Next<'a> {
    fn default() -> Self {
        Self {
            buf: (Cow::from(Vec::new()), 0),
            #[cfg(feature = "crossbeam")]
            stack: Stack::new(),
        }
    }
}

impl<'a, T: Into<Cow<'a, [u8]>>> From<T> for Next<'a> {
    fn from(value: T) -> Self {
        Self {
            buf: (value.into(), 0),
            #[cfg(feature = "crossbeam")]
            stack: Stack::new(),
        }
    }
}

/// The `NextTrace` struct provides middleware functionality for tracing and debugging serialization
/// and deserialization operations. It maintains a nested context to help identify and report errors
/// and their locations in the payload structure. Requires the `info` feature to be enabled.
#[cfg(feature = "info")]
#[derive(Clone, Debug)]
pub struct NextTrace<'a> {
    buf: (Cow<'a, [u8]>, usize), 
    depth: usize, 
    path: LinkedList<&'static str>,
    #[cfg(feature = "crossbeam")]
    stack: Stack<'a>
}

#[cfg(feature = "info")]
const MAX_NESTED_DEPTH: usize = 255;

#[cfg(feature = "info")]
impl<'a> NextTrace<'a> {
    pub fn from_mut(cow: &'a mut Cow<'_, [u8]>) -> Self {
        Self {
            buf: (Cow::Borrowed(&*cow), 0), 
            depth: MAX_NESTED_DEPTH, 
            path: LinkedList::new(),
            #[cfg(feature = "crossbeam")]
            stack: Stack::new(),
        }
    }

    pub fn with_mtu(mtu: usize) -> Self {
        Self {
            buf: (Cow::from(Vec::with_capacity(mtu)), 0), 
            depth: MAX_NESTED_DEPTH, 
            path: LinkedList::new(),
            #[cfg(feature = "crossbeam")]
            stack: Stack::new(),
        }
    }

    pub fn with_depth(depth: usize) -> Self {
        Self {
            buf: (Cow::from(Vec::new()), 0), 
            depth, 
            path: LinkedList::new(),
            #[cfg(feature = "crossbeam")]
            stack: Stack::new(),
        }
    }

    #[inline(always)]
    pub fn serialized(&self) -> Vec<u8> {
        self.buf.0.to_vec()
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[u8] { 
        self.buf.0.as_ref()
    }
}

#[cfg(feature = "info")]
impl<'a, T: Into<Cow<'a, [u8]>>> From<T> for NextTrace<'a> {
    fn from(value: T) -> Self {
        Self {
            buf: (value.into(), 0), 
            depth: MAX_NESTED_DEPTH, 
            path: LinkedList::new(),
            #[cfg(feature = "crossbeam")]
            stack: Stack::new(),
        }
    }
}

#[cfg(feature = "info")]
impl<'a> Default for NextTrace<'a> {
    fn default() -> Self {
        Self {
            buf: (Cow::from(Vec::new()), 0), 
            depth: MAX_NESTED_DEPTH, 
            path: LinkedList::new(),
            #[cfg(feature = "crossbeam")]
            stack: Stack::new(),
        }
    }
}

#[cfg(feature = "info")]
#[cfg(feature = "sync")]
impl<'a> Middleware<'a> for NextTrace<'a> {
    #[inline(always)]
    fn into_payload<C, T: IntoPayload<C>>(&mut self, value: &T, ctx: &mut C) -> Result<(), Error> {
        if self.path.len() > self.depth {
            return Err(Error::NestedDepthLimit(T::TYPE.to_string()))
        }

        self.path.push_back(T::TYPE);
 
        let path: Vec<&str> = self.path.iter().cloned().collect();
        println!("{}", path.join(" -> "));

        match value.into_payload(ctx, self) {
            Ok(value) => {
                self.path.pop_back();

                Ok(value)
            },
            Err(error) => {
                match error {
                    Error::Traced(_, _) => Err(error),
                    _ => {
                        let path: Vec<&str> = self.path.iter().cloned().collect();
                        Err(Error::Traced(error.to_string(), path.join(" -> ")))
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn from_payload<C, T: FromPayload<'a, C>>(&mut self, ctx: &mut C) -> Result<T, Error> {
        if self.path.len() > self.depth {
            return Err(Error::NestedDepthLimit(T::TYPE.to_string()))
        }

        self.path.push_back(T::TYPE);

        let path: Vec<&str> = self.path.iter().cloned().collect();
        println!("{}", path.join(" <- "));

        match T::from_payload(ctx, self) {
            Ok(value) => {
                self.path.pop_back();

                Ok(value)
            },
            Err(error) => {
                match error {
                    Error::Traced(_, _) => Err(error),
                    _ => {
                        let path: Vec<&str> = self.path.iter().cloned().collect();
                        Err(Error::Traced(error.to_string(), path.join(" <- ")))
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn write<T>(&mut self, data: &[T]) -> Result<(), Error> {
        self.buf.write(data)
    }

    #[inline(always)]
    fn read<T>(&mut self, nbytes: usize) -> Result<&'a [T], Error> {
        self.buf.read(nbytes)
    }

    #[inline(always)]
    fn read_mut<T>(&mut self, nbytes: usize) -> Result<&'a mut [T], Error> {
        self.buf.read_mut(nbytes)
    }

    #[cfg(feature = "crossbeam")]
    fn push<T: AnyBox<'a>>(&mut self, value: Box<T>) -> Result<&'a T, Error> {
        Ok(self.stack.push(value))
    }

    #[cfg(feature = "crossbeam")]
    fn push_mut<T: AnyBox<'a>>(&mut self, value: Box<T>) -> Result<&'a mut T, Error> {
        Ok(self.stack.push_mut(value))
    }

    #[cfg(feature = "crossbeam")]
    fn push_array<T: AnyBox<'a>>(&mut self, values: Box<[T]>) -> Result<&'a [T], Error> {
        Ok(self.stack.push_array(values))
    }

    #[cfg(feature = "crossbeam")]
    fn push_array_mut<T: AnyBox<'a>>(&mut self, values: Box<[T]>) -> Result<&'a mut [T], Error> {
        Ok(self.stack.push_array_mut(values))
    }

    #[cfg(not(feature = "crossbeam"))]
    #[inline(always)]
    fn push<T: AnyBox<'a>>(&mut self, _value: Box<T>) -> Result<&'a T, Error> {
        Err(Error::Stack("References disabled".to_string()))
    }

    #[cfg(not(feature = "crossbeam"))]
    #[inline(always)]
    fn push_mut<T: AnyBox<'a>>(&mut self, _value: Box<T>) -> Result<&'a mut T, Error> {
        Err(Error::Stack("References disabled".to_string()))
    }

    #[cfg(not(feature = "crossbeam"))]
    #[inline(always)]
    fn push_array<T: AnyBox<'a>>(&mut self, _values: Box<[T]>) -> Result<&'a [T], Error> {
        Err(Error::Stack("References disabled".to_string()))
    }

    #[cfg(not(feature = "crossbeam"))]
    #[inline(always)]
    fn push_array_mut<T: AnyBox<'a>>(&mut self, _values: Box<[T]>) -> Result<&'a mut [T], Error> {
        Err(Error::Stack("References disabled".to_string()))
    }
}

#[cfg(feature = "info")]
#[cfg(feature = "async")]
impl<'a> AsyncMiddleware<'a> for NextTrace<'a> {
    async fn poll_into_payload<C: Send + Sync, T: AsyncIntoPayload<C>>(
        &mut self,
        value: &T,
        ctx: &mut C
    ) -> Result<(), Error> {
        if self.path.len() > self.depth {
            return Err(Error::NestedDepthLimit(T::TYPE.to_string()));
        }

        self.path.push_back(T::TYPE);

        let path: Vec<&str> = self.path.iter().cloned().collect();
        println!("{}", path.join(" -> "));

        match value.poll_into_payload(ctx, self).await {
            Ok(value) => {
                self.path.pop_back();
                Ok(value)
            }
            Err(error) => {
                match error {
                    Error::Traced(_, _) => Err(error),
                    _ => {
                        let path: Vec<&str> = self.path.iter().cloned().collect();
                        Err(Error::Traced(error.to_string(), path.join(" -> ")))
                    }
                }
            }
        }
    }

    async fn poll_from_payload<C: Send + Sync, T: AsyncFromPayload<'a, C> + Send + Sync> (
        &mut self, 
        ctx: &mut C,
    ) -> Result<T, Error> {
        if self.path.len() > self.depth {
            return Err(Error::NestedDepthLimit(T::TYPE.to_string()));
        }

        self.path.push_back(T::TYPE);

        let path: Vec<&str> = self.path.iter().cloned().collect();
        println!("{}", path.join(" <- "));

        match T::poll_from_payload(ctx, self).await {
            Ok(value) => {
                self.path.pop_back();
                Ok(value)
            }
            Err(error) => {
                match error {
                    Error::Traced(_, _) => Err(error),
                    _ => {
                        let path: Vec<&str> = self.path.iter().cloned().collect();
                        Err(Error::Traced(error.to_string(), path.join(" <- ")))
                    }
                }
            }
        }
    }

    #[inline(always)]
    async fn poll_write<T>(&mut self, data: &[T]) -> Result<(), Error> {
        self.buf.write(data)
    }

    #[inline(always)]
    async fn poll_read<T: 'a>(&mut self, nbytes: usize) -> Result<&'a [T], Error> {
        self.buf.read(nbytes)
    }

    #[inline(always)]
    async fn poll_read_mut<T: 'a>(&mut self, nbytes: usize) -> Result<&'a mut [T], Error> {
        self.buf.read_mut(nbytes)
    }

    #[cfg(feature = "crossbeam")]
    #[inline(always)]
    async fn poll_push<T: AnyBox<'a>>(&mut self, value: Box<T>) -> Result<&'a T, Error> {
        Ok(self.stack.push(value))
    }

    #[cfg(feature = "crossbeam")]
    #[inline(always)]
    async fn poll_push_mut<T: AnyBox<'a>>(&mut self, value: Box<T>) -> Result<&'a mut T, Error> {
        Ok(self.stack.push_mut(value))
    }

    #[cfg(feature = "crossbeam")]
    #[inline(always)]
    async fn poll_push_array<T: AnyBox<'a>>(&mut self, values: Box<[T]>) -> Result<&'a [T], Error> {
        Ok(self.stack.push_array(values))
    }

    #[cfg(feature = "crossbeam")]
    #[inline(always)]
    async fn poll_push_array_mut<T: AnyBox<'a>>(&mut self, values: Box<[T]>) -> Result<&'a mut [T], Error> {
        Ok(self.stack.push_array_mut(values))
    }

    #[cfg(not(feature = "crossbeam"))]
    #[inline(always)]
    async fn poll_push<T: AnyBox<'a>>(&mut self, _value: Box<T>) -> Result<&'a T, Error> {
        Err(Error::Stack("References disabled".to_string()))
    }

    #[cfg(not(feature = "crossbeam"))]
    #[inline(always)]
    async fn poll_push_mut<T: AnyBox<'a>>(&mut self, _value: Box<T>) -> Result<&'a mut T, Error> {
        Err(Error::Stack("References disabled".to_string()))
    }

    #[cfg(not(feature = "crossbeam"))]
    #[inline(always)]
    async fn poll_push_array<T: AnyBox<'a>>(&mut self, _values: Box<[T]>) -> Result<&'a [T], Error> {
        Err(Error::Stack("References disabled".to_string()))
    }

    #[cfg(not(feature = "crossbeam"))]
    #[inline(always)]
    async fn poll_push_array_mut<T: AnyBox<'a>>(&mut self, _values: Box<[T]>) -> Result<&'a mut [T], Error> {
        Err(Error::Stack("References disabled".to_string()))
    }
}