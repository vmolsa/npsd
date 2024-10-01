
use core::borrow::{Borrow, BorrowMut};

use crate::AnyBox;
use super::{Error, Middleware, Payload, IntoPayload, FromPayload};

impl<'a, C, T: IntoPayload<C>> IntoPayload<C> for &'a T {
    #[inline]
    fn into_payload<'b, M: Middleware<'b>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload::<C, T>(&**self, ctx)
    }
}

impl<'a, C, T: FromPayload<'a, C> + Borrow<T> + AnyBox<'a>> FromPayload<'a, C> for &'a T {
    #[inline]
    fn from_payload<M: Middleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let value: T = next.from_payload(ctx)?;

        Ok(next.push(Box::new(value))?)
    }
}

impl<'a, C, T: Payload<'a, C> + Borrow<T> + AnyBox<'a>> Payload<'a, C> for &'a T {}

impl<'a, C, T: IntoPayload<C>> IntoPayload<C> for &'a mut T {
    #[inline]
    fn into_payload<'b, M: Middleware<'b>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload::<C, T>(&**self, ctx)
    }
}

impl<'a, C, T: FromPayload<'a, C> + BorrowMut<T> + AnyBox<'a>> FromPayload<'a, C> for &'a mut T {
    #[inline]
    fn from_payload<M: Middleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let value: T = next.from_payload(ctx)?;

        Ok(next.push_mut(Box::new(value))?)
    }
}

impl<'a, C, T: Payload<'a, C> + BorrowMut<T> + AnyBox<'a>> Payload<'a, C> for &'a mut T {}
