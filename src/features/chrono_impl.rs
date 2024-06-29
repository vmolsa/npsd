use chrono::prelude::*;

use crate::{Error, Middleware, PayloadContext, PayloadHandler, PayloadInfo, Payload, IntoPayload, FromPayload};

// TODO(): Impl
// use std::str::FromStr;
// impl PayloadInfo for NaiveDate {
//     const TYPE: &'static str = "NaiveDate";
//     const SIZE: Option<usize> = Some(core::mem::size_of::<NaiveDate>());
// }

// impl<C: PayloadContext> IntoPayload<C> for NaiveDate {
//     #[inline]
//     fn into_payload<M: Middleware>(&self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
//         next.into_payload(&self.to_string(), handler, ctx)
//     }
// }

// impl<'a, C: PayloadContext> FromPayload<'a, C> for NaiveDate {
//     #[inline]
//     fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
//         where
//             'a: 'b,
//     {
//         NaiveDate::from_str(next.from_payload(handler, ctx)?).map_err(|e| Error::Time(e.to_string()))
//     }
// }

// impl<'a, C: PayloadContext> Payload<'a, C> for NaiveDate {}

// impl PayloadInfo for NaiveTime {
//     const TYPE: &'static str = "NaiveTime";
//     const SIZE: Option<usize> = Some(core::mem::size_of::<NaiveTime>());
// }

// impl<C: PayloadContext> IntoPayload<C> for NaiveTime {
//     #[inline]
//     fn into_payload<M: Middleware>(&self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
//         next.into_payload(&self.to_string(), handler, ctx)
//     }
// }

// impl<'a, C: PayloadContext> FromPayload<'a, C> for NaiveTime {
//     #[inline]
//     fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
//         where
//             'a: 'b,
//     {
//         NaiveTime::from_str(next.from_payload(handler, ctx)?).map_err(|e| Error::Time(e.to_string()))
//     }
// }

// impl<'a, C: PayloadContext> Payload<'a, C> for NaiveTime {}

// impl PayloadInfo for NaiveDateTime {
//     const TYPE: &'static str = "NaiveDateTime";
//     const SIZE: Option<usize> = Some(core::mem::size_of::<NaiveDateTime>());
// }

// impl<C: PayloadContext> IntoPayload<C> for NaiveDateTime {
//     #[inline]
//     fn into_payload<M: Middleware>(&self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
//         next.into_payload(&self.to_string(), handler, ctx)
//     }
// }

// impl<'a, C: PayloadContext> FromPayload<'a, C> for NaiveDateTime {
//     #[inline]
//     fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
//         where
//             'a: 'b,
//     {
//         NaiveDateTime::from_str(next.from_payload(handler, ctx)?).map_err(|e| Error::Time(e.to_string()))
//     }
// }

// impl<'a, C: PayloadContext> Payload<'a, C> for NaiveDateTime {}

impl PayloadInfo for DateTime<Utc> {
    const TYPE: &'static str = "DateTime<Utc>";
    const SIZE: Option<usize> = Some(core::mem::size_of::<DateTime<Utc>>());
}

impl<C: PayloadContext> IntoPayload<C> for DateTime<Utc> {
    #[inline]
    fn into_payload<M: Middleware>(&self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&self.to_rfc3339(), handler, ctx)
    }
}

impl<'a, C: PayloadContext> FromPayload<'a, C> for DateTime<Utc> {
    #[inline]
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where
            'a: 'b,
    {
        DateTime::parse_from_rfc3339(next.from_payload(handler, ctx)?)
            .map_err(|e| Error::Time(e.to_string()))
            .map(|dt| dt.with_timezone(&Utc))
    }
}

impl<'a, C: PayloadContext> Payload<'a, C> for DateTime<Utc> {}

impl PayloadInfo for DateTime<Local> {
    const TYPE: &'static str = "DateTime<Local>";
    const SIZE: Option<usize> = Some(core::mem::size_of::<DateTime<Local>>());
}

impl<C: PayloadContext> IntoPayload<C> for DateTime<Local> {
    #[inline]
    fn into_payload<M: Middleware>(&self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&self.to_rfc3339(), handler, ctx)
    }
}

impl<'a, C: PayloadContext> FromPayload<'a, C> for DateTime<Local> {
    #[inline]
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
    where
        'a: 'b,
    {
        DateTime::parse_from_rfc3339(next.from_payload(handler, ctx)?)
            .map_err(|e| Error::Time(e.to_string()))
            .map(|dt| dt.with_timezone(&Local))
    }
}

impl<'a, C: PayloadContext> Payload<'a, C> for DateTime<Local> {}

impl PayloadInfo for DateTime<FixedOffset> {
    const TYPE: &'static str = "DateTime<FixedOffset>";
    const SIZE: Option<usize> = Some(core::mem::size_of::<DateTime<FixedOffset>>());
}

impl<C: PayloadContext> IntoPayload<C> for DateTime<FixedOffset> {
    #[inline]
    fn into_payload<M: Middleware>(&self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&self.to_rfc3339(), handler, ctx)
    }
}

impl<'a, C: PayloadContext> FromPayload<'a, C> for DateTime<FixedOffset> {
    #[inline]
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
    where
        'a: 'b,
    {
        DateTime::parse_from_rfc3339(next.from_payload(handler, ctx)?)
            .map_err(|e| Error::Time(e.to_string()))
            .map(|dt| dt.with_timezone(dt.offset()))
    }
}

impl<'a, C: PayloadContext> Payload<'a, C> for DateTime<FixedOffset> {}