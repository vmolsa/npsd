
use core::borrow::{Borrow, BorrowMut};
use super::{Error, AsyncMiddleware, AsyncPayload, AsyncIntoPayload, AsyncFromPayload};

impl<'a, C: Send + Sync, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for &'a T {
    #[inline]
    async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.poll_into_payload::<C, T>(&**self, ctx).await
    }
}

impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C> + Borrow<T>> AsyncFromPayload<'a, C> for &'a T {
    #[inline]
    async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let value: T = next.poll_from_payload(ctx).await?;

        next.poll_push(Box::new(value)).await
    }
}

impl<'a, C: Send + Sync, T: AsyncPayload<'a, C> + Borrow<T>> AsyncPayload<'a, C> for &'a T {}

impl<C: Send + Sync, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for &mut T {
    #[inline]
    async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.poll_into_payload::<C, T>(&**self, ctx).await
    }
}

impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C> + BorrowMut<T>> AsyncFromPayload<'a, C> for &'a mut T {
    #[inline]
    async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let value: T = next.poll_from_payload(ctx).await?;

        next.poll_push_mut(Box::new(value)).await
    }
}

impl<'a, C: Send + Sync, T: AsyncPayload<'a, C> + BorrowMut<T>> AsyncPayload<'a, C> for &'a mut T {}
