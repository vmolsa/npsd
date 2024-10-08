use core::mem;
use std::ops::Range;

use super::{Error, AsyncMiddleware, AsyncPayload, AsyncIntoPayload, AsyncFromPayload};

impl<'a, C: Send + Sync, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for &'a [T] {
    async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
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

impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C> + 'a> AsyncFromPayload<'a, C> for &'a [T] {
    async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let len: usize = next.poll_from_payload(ctx).await?;

        if mem::size_of::<T>() == 1 {
            next.poll_read(len).await
        } else {
            let mut vec = Vec::with_capacity(len);

            for _ in 0..len {
                vec.push(next.poll_from_payload::<C, T>(ctx).await?);
            }

            next.poll_push_array(vec.into_boxed_slice()).await
        }
    }
}

impl<'a, C: Send + Sync, T: AsyncPayload<'a, C>> AsyncPayload<'a, C> for &'a [T] {}

impl<C: Send + Sync, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for &mut [T] {
    #[inline]
    async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.poll_into_payload::<C, &[T]>(&self.as_ref(), ctx).await
    }
}

impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for &'a mut [T] where T: Clone {
    async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        if mem::size_of::<T>() == 1 {
            let nbytes: usize = next.poll_from_payload(ctx).await?;

            next.poll_read_mut(nbytes).await
        } else {
            let len: usize = next.poll_from_payload(ctx).await?;
            let mut vec = Vec::with_capacity(len);

            for _ in 0..len {
                vec.push(next.poll_from_payload::<C, T>(ctx).await?);
            }

            next.poll_push_array_mut(vec.into_boxed_slice()).await
        }
    }
}

impl<'a, C: Send + Sync, T: AsyncPayload<'a, C>> AsyncPayload<'a, C> for &'a mut [T] 
    where T: Clone {}

impl<C: Send + Sync, T: AsyncIntoPayload<C>, const N: usize> AsyncIntoPayload<C> for [T; N] {
    async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
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

impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C> + Default + 'a, const N: usize> AsyncFromPayload<'a, C> for [T; N] 
    where T: Copy
{
    async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let mut result = [T::default(); N];

        if mem::size_of::<T>() == 1 {
            let bytes: &[T] = next.poll_read(N).await?;
            result.copy_from_slice(bytes);
        } else {
            for i in 0..N {
                result[i] = next.poll_from_payload(ctx).await?;
            }
        }

        Ok(result)
    }
}

impl<'a, C: Send + Sync, T: AsyncPayload<'a, C> + Default, const N: usize> AsyncPayload<'a, C> for [T; N] where T: Copy {}

impl<C: Send + Sync, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for Range<T> {
    async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.poll_into_payload(&self.start, ctx).await?;
        next.poll_into_payload(&self.end, ctx).await
    }
}

impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for Range<T> {
    async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let start: T = next.poll_from_payload(ctx).await?;
        let end: T = next.poll_from_payload(ctx).await?;

        Ok(Range { start, end })
    }
}

impl<'a, C: Send + Sync, T: AsyncPayload<'a, C>> AsyncPayload<'a, C> for Range<T> {}