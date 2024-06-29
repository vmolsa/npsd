use crate::{Error as PayloadError, Middleware, PayloadContext, PayloadHandler, PayloadInfo, Payload, IntoPayload, FromPayload};
use uuid::*;

impl<C: PayloadContext> IntoPayload<C> for Uuid {
    #[inline]
    fn into_payload<M: Middleware>(&self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), PayloadError> {
        next.into_payload(&self.as_u128(), handler, ctx)
    }
}

impl<'a, C: PayloadContext> FromPayload<'a, C> for Uuid {
    #[inline]
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, PayloadError>
        where
            'a: 'b,
    {
        Ok(Uuid::from_u128(next.from_payload(handler, ctx)?))
    }
}

impl<'a, C: PayloadContext> Payload<'a, C> for Uuid {}

impl PayloadInfo for Uuid {
    const HASH: u64 = <u128 as PayloadInfo>::HASH;
    const TYPE: &'static str = "Uuid";
    const SIZE: Option<usize> = u128::SIZE;
}