use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use super::{Error, Middleware, PayloadContext, PayloadHandler, PayloadInfo, Payload, IntoPayload, FromPayload};

impl<'a, C: PayloadContext> IntoPayload<C> for Duration {
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        let secs = self.as_secs();
        let nanos = self.subsec_nanos();

        next.into_payload(&(secs, nanos), handler, ctx)
    }
}

impl<'a, C: PayloadContext> FromPayload<'a, C> for Duration {
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        let (secs, nanos): (u64, u32) = next.from_payload(handler, ctx)?;

        Ok(Duration::new(secs, nanos))
    }
}

impl<'a, C: PayloadContext> Payload<'a, C> for Duration {}

impl PayloadInfo for Duration {
    const TYPE: &'static str = "Duration";
    const SIZE: Option<usize> = Some(std::mem::size_of::<Duration>());
}

impl<'a, C: PayloadContext> IntoPayload<C> for Instant {
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(time) => next.into_payload(&time, handler, ctx),
            Err(error) => Err(Error::Time(error.to_string())),
        }
    }
}

impl PayloadInfo for Instant {
    const TYPE: &'static str = "Instant";
    const SIZE: Option<usize> = Some(std::mem::size_of::<Instant>());
}

impl<'a, C: PayloadContext> IntoPayload<C> for SystemTime {
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(time) => next.into_payload(&time, handler, ctx),
            Err(error) => Err(Error::Time(error.to_string())),
        }
    }
}

impl<'a, C: PayloadContext> FromPayload<'a, C> for SystemTime {
    #[inline]
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(UNIX_EPOCH + next.from_payload::<C, Duration>(handler, ctx)?)
    }
}

impl<'a, C: PayloadContext> Payload<'a, C> for SystemTime {}

impl PayloadInfo for SystemTime {
    const TYPE: &'static str = "SystemTime";
    const SIZE: Option<usize> = Some(std::mem::size_of::<SystemTime>());
}
