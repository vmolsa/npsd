use super::{Error, FromPayload, IntoPayload, Middleware, Payload};

impl<'a, C, T: IntoPayload<C>> IntoPayload<C> for Option<T> {
    fn into_payload<'b, M: Middleware<'b>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        if let Some(data) = self {
            next.into_payload(&1u8, ctx)?;
            next.into_payload(data, ctx)
        } else {
            next.into_payload(&0u8, ctx)
        }
    }
}

impl<'a, C, T: FromPayload<'a, C>> FromPayload<'a, C> for Option<T> {
    fn from_payload<M: Middleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let byte: u8 = next.from_payload(ctx)?;

        if byte != 0 {
            let res: T = next.from_payload(ctx)?;

            Ok(Some(res))
        } else {
            Ok(None)
        }
    }
}

impl<'a, C, T: Payload<'a, C>> Payload<'a, C> for Option<T> {}

impl<'a, C, T: IntoPayload<C>, E: IntoPayload<C>> IntoPayload<C> for Result<T, E> {
    fn into_payload<'b, M: Middleware<'b>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        match self {
            Ok(res) => {
                next.into_payload(&1u8, ctx)?;
                next.into_payload(res, ctx)
            },
            Err(error) => {
                next.into_payload(&0u8, ctx)?;
                next.into_payload(error, ctx)
            }
        }
    }
}

impl<'a, C, T: FromPayload<'a, C>, E: FromPayload<'a, C>> FromPayload<'a, C> for Result<T, E> {
    fn from_payload<M: Middleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let byte: u8 = next.from_payload(ctx)?;

        if byte != 0 {
            let res: T = next.from_payload(ctx)?;

            Ok(Ok(res))
        } else {
            let res: E = next.from_payload(ctx)?;

            Ok(Err(res))
        }
    }
}

impl<'a, C, T: Payload<'a, C>, E: Payload<'a, C>> Payload<'a, C> for Result<T, E> {}
