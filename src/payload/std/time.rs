use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use super::{Error, Middleware, Payload, IntoPayload, FromPayload};

impl<C> IntoPayload<C> for Duration {
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        let secs = self.as_secs();
        let nanos = self.subsec_nanos();

        next.into_payload(&(secs, nanos), ctx)
    }
}

impl<'a, C> FromPayload<'a, C> for Duration {
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        let (secs, nanos): (u64, u32) = next.from_payload(ctx)?;

        Ok(Duration::new(secs, nanos))
    }
}

impl<C> Payload<C> for Duration {}

impl<C> IntoPayload<C> for Instant {
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(time) => next.into_payload(&time, ctx),
            Err(error) => Err(Error::Time(error.to_string())),
        }
    }
}

impl<C> IntoPayload<C> for SystemTime {
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(time) => next.into_payload(&time, ctx),
            Err(error) => Err(Error::Time(error.to_string())),
        }
    }
}

impl<'a, C> FromPayload<'a, C> for SystemTime {
    #[inline]
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(UNIX_EPOCH + next.from_payload::<C, Duration>(ctx)?)
    }
}

impl<C> Payload<C> for SystemTime {}
