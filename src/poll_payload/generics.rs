
use core::borrow::{Borrow, BorrowMut};
use super::{Error, AsyncMiddleware, AsyncPayload, AsyncIntoPayload, AsyncFromPayload};

impl<'a, C: Send + Sync, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for &'a T {
    #[inline]
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
        next.poll_into_payload::<C, T>(&**self, ctx).await
    }
}

impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C> + Borrow<T>> AsyncFromPayload<'a, C> for &'a T {
    #[inline]
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        // TODO(): Replace Box::leak()
        Ok(Box::leak(Box::new(next.poll_from_payload::<C, T>(ctx).await?)))
    }
}

impl<'a, C: Send + Sync, T: AsyncPayload<C> + Borrow<T>> AsyncPayload<C> for &'a T {}

impl<C: Send + Sync, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for &mut T {
    #[inline]
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
        next.poll_into_payload::<C, T>(&**self, ctx).await
    }
}

impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C> + BorrowMut<T>> AsyncFromPayload<'a, C> for &'a mut T {
    #[inline]
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        // TODO(): Replace Box::leak()
        Ok(Box::leak(Box::new(next.poll_from_payload::<C, T>(ctx).await?)))
    }
}

impl<'a, C: Send + Sync, T: AsyncPayload<C> + BorrowMut<T>> AsyncPayload<C> for &'a mut T {}
