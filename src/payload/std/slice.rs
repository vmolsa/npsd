use core::{mem, slice};
use std::borrow::Cow;

use crate::payload::size_mul;
use super::{Error, Middleware, PayloadContext, PayloadHandler, PayloadInfo, Payload, IntoPayload, FromPayload, PayloadConstHash};

impl<'a, C: PayloadContext, T: IntoPayload<C> + PayloadInfo> IntoPayload<C> for &[T] {
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        if mem::size_of::<T>() == 1 {
            next.into_payload(&self.len(), handler, ctx)?;
            next.write(handler, self)?;
        } else {
            next.into_payload(&self.len(), handler, ctx)?;

            for elem in *self {
                next.into_payload(elem, handler, ctx)?;
            }
        }

        Ok(())
    }
}

impl<'a, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo> FromPayload<'a, C> for &'a [T] {
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error> 
        where 'a: 'b,
    {
        let len: usize = next.from_payload(handler, ctx)?;

        if mem::size_of::<T>() == 1 {
            Ok(next.read(handler, len)?)
        } else {
            let mut vec = Vec::with_capacity(len);

            for _ in 0..len {
                vec.push(next.from_payload::<C, T>(handler, ctx)?);
            }

            Ok(Box::leak(vec.into_boxed_slice()))
        }
    }
}

impl<'a, C: PayloadContext, T: Payload<'a, C> + PayloadInfo> Payload<'a, C> for &'a [T] {}

impl<'a, T: PayloadInfo> PayloadInfo for &'a [T] {
    const HASH: u64 = PayloadConstHash(stringify!(&[T]).as_bytes()) ^ T::HASH;
    const TYPE: &'static str = "&[T]";
}

impl<'a, C: PayloadContext, T: IntoPayload<C> + PayloadInfo> IntoPayload<C> for &mut [T] {
    #[inline]
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload::<C, &[T]>(&self.as_ref(), handler, ctx)
    }
}

impl<'a, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo> FromPayload<'a, C> for &'a mut [T] where T: Clone {
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        if mem::size_of::<T>() == 1 {
            let mut slice: Cow<'a, [T]> = next.from_payload(handler, ctx)?;

            let result = unsafe {
                slice::from_raw_parts_mut(slice.to_mut().as_mut_ptr() as *mut T, slice.len())
            };

            mem::forget(slice);

            Ok(result)
        } else {
            let len: usize = next.from_payload(handler, ctx)?;
            let mut vec = Vec::with_capacity(len);

            for _ in 0..len {
                vec.push(next.from_payload::<C, T>(handler, ctx)?);
            }

            Ok(Box::leak(vec.into_boxed_slice()))
        }
    }
}

impl<'a, C: PayloadContext, T: Payload<'a, C> + PayloadInfo> Payload<'a, C> for &'a mut [T] 
    where T: Clone {}

impl<'a, T: PayloadInfo> PayloadInfo for &'a mut [T] {
    const HASH: u64 = PayloadConstHash(stringify!(&[T]).as_bytes()) ^ T::HASH;
    const TYPE: &'static str = "&mut [T]";
}

impl<'a, C: PayloadContext, T: IntoPayload<C> + PayloadInfo, const N: usize> IntoPayload<C> for [T; N] {
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        if mem::size_of::<T>() == 1 {
            next.write(handler, self)?;
        } else {
            for elem in self.into_iter() {
                next.into_payload(elem, handler, ctx)?;
            }
        }
        
        Ok(())
    }
}

impl<'a, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo + 'a, const N: usize> FromPayload<'a, C> for [T; N] 
    where T: Copy
{
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        if mem::size_of::<T>() == 1 {
            let bytes: &[T] = next.read(handler, N)?;

            Ok(unsafe {
                *(bytes.as_ptr() as *const [T; N])
            })
        } else {
            let mut vec = Vec::with_capacity(N);

            for _ in 0..N {
                vec.push(next.from_payload::<C, T>(handler, ctx)?);
            }

            Ok(unsafe { *(vec.leak().as_ptr() as *const [T; N]) })
        }
    }
}

impl<'a, C: PayloadContext, T: Payload<'a, C> + PayloadInfo + 'a, const N: usize> Payload<'a, C> for [T; N] where T: Copy {}

impl<T: PayloadInfo, const N: usize> PayloadInfo for [T; N]  {
    const HASH: u64 = PayloadConstHash(stringify!(&[T]).as_bytes()) ^ N as u64 ^ T::HASH;
    const TYPE: &'static str = "[T; N] ";
    const SIZE: Option<usize> = size_mul(T::SIZE, N);
}