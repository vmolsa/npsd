
use core::borrow::{Borrow, BorrowMut};

use super::{Error, Middleware, PayloadContext, PayloadHandler, PayloadInfo, Payload, IntoPayload, FromPayload};

impl<C: PayloadContext, T: IntoPayload<C> + PayloadInfo> IntoPayload<C> for &T {
    #[inline]
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload::<C, T>(&**self, handler, ctx)
    }
}

impl<'a, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo + Borrow<T>> FromPayload<'a, C> for &'a T {
    #[inline]
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(Box::leak(Box::new(next.from_payload::<C, T>(handler, ctx)?)))
    }
}

impl<'a, C: PayloadContext, T: Payload<'a, C> + PayloadInfo + Borrow<T>> Payload<'a, C> for &'a T {}

impl<T: PayloadInfo> PayloadInfo for &T {
    const HASH: u64 = T::HASH;
    const TYPE: &'static str = T::TYPE;
    const SIZE: Option<usize> = T::SIZE;
}

impl<C: PayloadContext, T: IntoPayload<C> + PayloadInfo> IntoPayload<C> for &mut T {
    #[inline]
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload::<C, T>(&**self, handler, ctx)
    }
}

impl<'a, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo + BorrowMut<T>> FromPayload<'a, C> for &'a mut T {
    #[inline]
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(Box::leak(Box::new(next.from_payload::<C, T>(handler, ctx)?)))
    }
}

impl<'a, C: PayloadContext, T: Payload<'a, C> + PayloadInfo + BorrowMut<T>> Payload<'a, C> for &'a mut T {}

impl<T: PayloadInfo> PayloadInfo for &mut T {
    const HASH: u64 = T::HASH;
    const TYPE: &'static str = T::TYPE;
    const SIZE: Option<usize> = T::SIZE;
}