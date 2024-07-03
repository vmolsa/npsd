
#[cfg(feature = "sync")]
use crate::{Middleware, Payload, IntoPayload, FromPayload};

#[cfg(feature = "async")]
use crate::{AsyncMiddleware, AsyncPayload, AsyncIntoPayload, AsyncFromPayload};

use crate::{PayloadInfo, Error as PayloadError};

use uuid::*;

impl PayloadInfo for Uuid {
    const HASH: u64 = <u128 as PayloadInfo>::HASH;
    const TYPE: &'static str = "Uuid";
    const SIZE: Option<usize> = u128::SIZE;
}

#[cfg(feature = "sync")]
impl<C> IntoPayload<C> for Uuid {
    #[inline]
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), PayloadError> {
        next.into_payload(&self.as_u128(), ctx)
    }
}

#[cfg(feature = "sync")]
impl<'a, C> FromPayload<'a, C> for Uuid {
    #[inline]
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, PayloadError>
        where
            'a: 'b,
    {
        Ok(Uuid::from_u128(next.from_payload(ctx)?))
    }
}

#[cfg(feature = "sync")]
impl<C> Payload<C> for Uuid {}

#[cfg(feature = "async")]
impl<C> AsyncPayload<C> for Uuid {}

#[cfg(feature = "async")]
impl<C> AsyncIntoPayload<C> for Uuid {
    #[inline]
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), PayloadError> {
        next.poll_into_payload(&self.as_u128(), ctx).await
    }
}

#[cfg(feature = "async")]
impl<'a, C> AsyncFromPayload<'a, C> for Uuid {
    #[inline]
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, PayloadError>
        where
            'a: 'b,
    {
        Ok(Uuid::from_u128(next.poll_from_payload(ctx).await?))
    }
}
