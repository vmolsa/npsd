
use core::borrow::{Borrow, BorrowMut};

use super::{Error, Middleware, Payload, IntoPayload, FromPayload};

impl<'a, C, T: IntoPayload<C>> IntoPayload<C> for &'a T {
    #[inline]
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload::<C, T>(&**self, ctx)
    }
}

impl<'a, C, T: FromPayload<'a, C> + Borrow<T>> FromPayload<'a, C> for &'a T {
    #[inline]
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        // TODO(): Replace Box::leak()
        Ok(Box::leak(Box::new(next.from_payload::<C, T>(ctx)?)))
    }
}

impl<'a, C, T: Payload<C> + Borrow<T>> Payload<C> for &'a T {}

impl<'a, C, T: IntoPayload<C>> IntoPayload<C> for &'a mut T {
    #[inline]
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload::<C, T>(&**self, ctx)
    }
}

impl<'a, C, T: FromPayload<'a, C> + BorrowMut<T>> FromPayload<'a, C> for &'a mut T {
    #[inline]
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        // TODO(): Replace Box::leak()
        Ok(Box::leak(Box::new(next.from_payload::<C, T>(ctx)?)))
    }
}

impl<'a, C, T: Payload<C> + BorrowMut<T>> Payload<C> for &'a mut T {}
