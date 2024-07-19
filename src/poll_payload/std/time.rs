use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use super::{Error, AsyncMiddleware, AsyncPayload, AsyncIntoPayload, AsyncFromPayload};

impl<C: Send + Sync> AsyncIntoPayload<C> for Duration {
    async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        let secs = self.as_secs();
        let nanos = self.subsec_nanos();

        next.poll_into_payload(&(secs, nanos), ctx).await
    }
}

impl<'a, C: Send + Sync> AsyncFromPayload<'a, C> for Duration {
    async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let (secs, nanos): (u64, u32) = next.poll_from_payload(ctx).await?;

        Ok(Duration::new(secs, nanos))
    }
}

impl<'a, C: Send + Sync> AsyncPayload<'a, C> for Duration {}

impl<C: Send + Sync> AsyncIntoPayload<C> for Instant {
    async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(time) => next.poll_into_payload(&time, ctx).await,
            Err(error) => Err(Error::Time(error.to_string())),
        }
    }
}

impl<C: Send + Sync> AsyncIntoPayload<C> for SystemTime {
    async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(time) => next.poll_into_payload(&time, ctx).await,
            Err(error) => Err(Error::Time(error.to_string())),
        }
    }
}

impl<'a, C: Send + Sync> AsyncFromPayload<'a, C> for SystemTime {
    #[inline]
    async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        Ok(UNIX_EPOCH + next.poll_from_payload::<C, Duration>(ctx).await?)
    }
}

impl<'a, C: Send + Sync> AsyncPayload<'a, C> for SystemTime {}
