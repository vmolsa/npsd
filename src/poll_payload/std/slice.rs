use core::{mem, slice};
use std::borrow::Cow;

use super::{Error, AsyncMiddleware, AsyncPayload, AsyncIntoPayload, AsyncFromPayload};

impl<'a, C, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for &'a [T] {
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
        if mem::size_of::<T>() == 1 {
            next.poll_into_payload(&self.len(), ctx).await?;
            next.poll_write(self).await?;
        } else {
            next.poll_into_payload(&self.len(), ctx).await?;

            for elem in *self {
                next.poll_into_payload(elem, ctx).await?;
            }
        }

        Ok(())
    }
}

impl<'a, C, T: AsyncFromPayload<'a, C> + 'a> AsyncFromPayload<'a, C> for &[T] {
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error> 
        where 'a: 'b,
    {
        let len: usize = next.poll_from_payload(ctx).await?;

        if mem::size_of::<T>() == 1 {
            Ok(next.poll_read(len).await?)
        } else {
            let mut vec = Vec::with_capacity(len);

            for _ in 0..len {
                vec.push(next.poll_from_payload::<C, T>(ctx).await?);
            }

            Ok(Box::leak(vec.into_boxed_slice()))
        }
    }
}

impl<'a, C, T: AsyncPayload<C>> AsyncPayload<C> for &'a [T] {}

impl<C, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for &mut [T] {
    #[inline]
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
        next.poll_into_payload::<C, &[T]>(&self.as_ref(), ctx).await
    }
}

impl<'a, C, T: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for &'a mut [T] where T: Clone {
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        if mem::size_of::<T>() == 1 {
            let mut slice: Cow<'a, [T]> = next.poll_from_payload(ctx).await?;

            let result = unsafe {
                slice::from_raw_parts_mut(slice.to_mut().as_mut_ptr() as *mut T, slice.len())
            };

            mem::forget(slice);

            Ok(result)
        } else {
            let len: usize = next.poll_from_payload(ctx).await?;
            let mut vec = Vec::with_capacity(len);

            for _ in 0..len {
                vec.push(next.poll_from_payload::<C, T>(ctx).await?);
            }

            Ok(Box::leak(vec.into_boxed_slice()))
        }
    }
}

impl<'a, C, T: AsyncPayload<C>> AsyncPayload<C> for &'a mut [T] 
    where T: Clone {}

impl<C, T: AsyncIntoPayload<C>, const N: usize> AsyncIntoPayload<C> for [T; N] {
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
        if mem::size_of::<T>() == 1 {
            next.poll_write(self).await?;
        } else {
            for elem in self.into_iter() {
                next.poll_into_payload(elem, ctx).await?;
            }
        }
        
        Ok(())
    }
}

impl<'a, C, T: AsyncFromPayload<'a, C> + 'a, const N: usize> AsyncFromPayload<'a, C> for [T; N] 
    where T: Copy
{
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        if mem::size_of::<T>() == 1 {
            let bytes: &[T] = next.poll_read(N).await?;

            Ok(unsafe {
                *(bytes.as_ptr() as *const [T; N])
            })
        } else {
            let mut vec = Vec::with_capacity(N);

            for _ in 0..N {
                vec.push(next.poll_from_payload::<C, T>(ctx).await?);
            }

            Ok(unsafe { *(vec.leak().as_ptr() as *const [T; N]) })
        }
    }
}

impl<C, T: AsyncPayload<C>, const N: usize> AsyncPayload<C> for [T; N] where T: Copy {}