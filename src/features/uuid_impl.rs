
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
impl<C> IntoPayload<C>  for Uuid {
    #[inline]
    fn into_payload<'m, M: Middleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), PayloadError> {
        next.into_payload(&self.as_u128(), ctx)
    }
}

#[cfg(feature = "sync")]
impl<'a, C> FromPayload<'a, C> for Uuid {
    #[inline]
    fn from_payload<M: Middleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, PayloadError> {
        Ok(Uuid::from_u128(next.from_payload(ctx)?))
    }
}

#[cfg(feature = "sync")]
impl<'a, C> Payload<'a, C> for Uuid {}

#[cfg(feature = "async")]
impl<'a, C: Send + Sync> AsyncPayload<'a, C> for Uuid {}

#[cfg(feature = "async")]
impl<C: Send + Sync> AsyncIntoPayload<C> for Uuid {
    #[inline]
    async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), PayloadError> {
        next.poll_into_payload(&self.as_u128(), ctx).await
    }
}

#[cfg(feature = "async")]
impl<'a, C: Send + Sync> AsyncFromPayload<'a, C> for Uuid {
    #[inline]
    async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, PayloadError> {
        Ok(Uuid::from_u128(next.poll_from_payload(ctx).await?))
    }
}
