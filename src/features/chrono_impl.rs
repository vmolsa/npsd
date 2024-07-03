use chrono::prelude::*;

#[cfg(feature = "sync")]
use crate::{Middleware, Payload, IntoPayload, FromPayload};

#[cfg(feature = "async")]
use crate::{AsyncMiddleware, AsyncPayload, AsyncIntoPayload, AsyncFromPayload};

use crate::{Error, PayloadInfo};

// TODO(): Impl
// use std::str::FromStr;
// impl PayloadInfo for NaiveDate {
//     const TYPE: &'static str = "NaiveDate";
//     const SIZE: Option<usize> = Some(core::mem::size_of::<NaiveDate>());
// }

// impl<C> IntoPayload<C> for NaiveDate {
//     #[inline]
//     fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
//         next.into_payload(&self.to_string(), ctx)
//     }
// }

// impl<'a, C> FromPayload<'a, C> for NaiveDate {
//     #[inline]
//     fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
//         where
//             'a: 'b,
//     {
//         NaiveDate::from_str(next.from_payload(ctx)?).map_err(|e| Error::Time(e.to_string()))
//     }
// }

// impl<'a, C> Payload<C> for NaiveDate {}

// impl PayloadInfo for NaiveTime {
//     const TYPE: &'static str = "NaiveTime";
//     const SIZE: Option<usize> = Some(core::mem::size_of::<NaiveTime>());
// }

// impl<C> IntoPayload<C> for NaiveTime {
//     #[inline]
//     fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
//         next.into_payload(&self.to_string(), ctx)
//     }
// }

// impl<'a, C> FromPayload<'a, C> for NaiveTime {
//     #[inline]
//     fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
//         where
//             'a: 'b,
//     {
//         NaiveTime::from_str(next.from_payload(ctx)?).map_err(|e| Error::Time(e.to_string()))
//     }
// }

// impl<'a, C> Payload<C> for NaiveTime {}

// impl PayloadInfo for NaiveDateTime {
//     const TYPE: &'static str = "NaiveDateTime";
//     const SIZE: Option<usize> = Some(core::mem::size_of::<NaiveDateTime>());
// }

// impl<C> IntoPayload<C> for NaiveDateTime {
//     #[inline]
//     fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
//         next.into_payload(&self.to_string(), ctx)
//     }
// }

// impl<'a, C> FromPayload<'a, C> for NaiveDateTime {
//     #[inline]
//     fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
//         where
//             'a: 'b,
//     {
//         NaiveDateTime::from_str(next.from_payload(ctx)?).map_err(|e| Error::Time(e.to_string()))
//     }
// }

// impl<'a, C> Payload<C> for NaiveDateTime {}

impl PayloadInfo for DateTime<Utc> {
    const TYPE: &'static str = "DateTime<Utc>";
    const SIZE: Option<usize> = Some(core::mem::size_of::<DateTime<Utc>>());
}

#[cfg(feature = "sync")]
impl<C> IntoPayload<C> for DateTime<Utc> {
    #[inline]
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&self.to_rfc3339(), ctx)
    }
}

#[cfg(feature = "sync")]
impl<'a, C> FromPayload<'a, C> for DateTime<Utc> {
    #[inline]
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where
            'a: 'b,
    {
        DateTime::parse_from_rfc3339(next.from_payload(ctx)?)
            .map_err(|e| Error::Time(e.to_string()))
            .map(|dt| dt.with_timezone(&Utc))
    }
}

#[cfg(feature = "sync")]
impl<'a, C> Payload<C> for DateTime<Utc> {}

#[cfg(feature = "async")]
impl<C> AsyncIntoPayload<C> for DateTime<Utc> {
    #[inline]
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
        next.poll_into_payload(&self.to_rfc3339(), ctx).await
    }
}

#[cfg(feature = "async")]
impl<'a, C> AsyncFromPayload<'a, C> for DateTime<Utc> {
    #[inline]
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where
            'a: 'b,
    {
        DateTime::parse_from_rfc3339(next.poll_from_payload(ctx).await?)
            .map_err(|e| Error::Time(e.to_string()))
            .map(|dt| dt.with_timezone(&Utc))
    }
}

#[cfg(feature = "async")]
impl<C> AsyncPayload<C> for DateTime<Utc> {}

impl PayloadInfo for DateTime<Local> {
    const TYPE: &'static str = "DateTime<Local>";
    const SIZE: Option<usize> = Some(core::mem::size_of::<DateTime<Local>>());
}

#[cfg(feature = "sync")]
impl<C> IntoPayload<C> for DateTime<Local> {
    #[inline]
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&self.to_rfc3339(), ctx)
    }
}

#[cfg(feature = "sync")]
impl<'a, C> FromPayload<'a, C> for DateTime<Local> {
    #[inline]
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
    where
        'a: 'b,
    {
        DateTime::parse_from_rfc3339(next.from_payload(ctx)?)
            .map_err(|e| Error::Time(e.to_string()))
            .map(|dt| dt.with_timezone(&Local))
    }
}

#[cfg(feature = "sync")]
impl<C> Payload<C> for DateTime<Local> {}

#[cfg(feature = "async")]
impl<C> AsyncIntoPayload<C> for DateTime<Local> {
    #[inline]
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
        next.poll_into_payload(&self.to_rfc3339(), ctx).await
    }
}

#[cfg(feature = "async")]
impl<'a, C> AsyncFromPayload<'a, C> for DateTime<Local> {
    #[inline]
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where
            'a: 'b,
    {
        DateTime::parse_from_rfc3339(next.poll_from_payload(ctx).await?)
            .map_err(|e| Error::Time(e.to_string()))
            .map(|dt| dt.with_timezone(&Local))
    }
}
#[cfg(feature = "async")]
impl<C> AsyncPayload<C> for DateTime<Local> {}

impl PayloadInfo for DateTime<FixedOffset> {
    const TYPE: &'static str = "DateTime<FixedOffset>";
    const SIZE: Option<usize> = Some(core::mem::size_of::<DateTime<FixedOffset>>());
}

#[cfg(feature = "sync")]
impl<C> IntoPayload<C> for DateTime<FixedOffset> {
    #[inline]
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&self.to_rfc3339(), ctx)
    }
}

#[cfg(feature = "sync")]
impl<'a, C> FromPayload<'a, C> for DateTime<FixedOffset> {
    #[inline]
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
    where
        'a: 'b,
    {
        DateTime::parse_from_rfc3339(next.from_payload(ctx)?)
            .map_err(|e| Error::Time(e.to_string()))
            .map(|dt| dt.with_timezone(dt.offset()))
    }
}

#[cfg(feature = "sync")]
impl<C> Payload<C> for DateTime<FixedOffset> {}

#[cfg(feature = "async")]
impl<C> AsyncIntoPayload<C> for DateTime<FixedOffset> {
    #[inline]
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
        next.poll_into_payload(&self.to_rfc3339(), ctx).await
    }
}

#[cfg(feature = "async")]
impl<'a, C> AsyncFromPayload<'a, C> for DateTime<FixedOffset> {
    #[inline]
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where
            'a: 'b,
    {
        DateTime::parse_from_rfc3339(next.poll_from_payload(ctx).await?)
            .map_err(|e| Error::Time(e.to_string()))
            .map(|dt| dt.with_timezone(dt.offset()))
    }
}

#[cfg(feature = "async")]
impl<C> AsyncPayload<C> for DateTime<FixedOffset> {}
