use super::{size_add, size_max, Error, FromPayload, IntoPayload, Middleware, Payload, PayloadConstHash, PayloadContext, PayloadHandler, PayloadInfo};

impl<'a, C: PayloadContext, T: IntoPayload<C> + PayloadInfo> IntoPayload<C> for Option<T> {
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        if let Some(data) = self {
            next.into_payload(&1u8, handler, ctx)?;
            next.into_payload(data, handler, ctx)
        } else {
            next.into_payload(&0u8, handler, ctx)
        }
    }
}

impl<'a, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo> FromPayload<'a, C> for Option<T> {
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b
    {
        let byte: u8 = next.from_payload(handler, ctx)?;

        if byte != 0 {
            let res: T = next.from_payload(handler, ctx)?;

            Ok(Some(res))
        } else {
            Ok(None)
        }
    }
}

impl<'a, C: PayloadContext, T: Payload<'a, C> + PayloadInfo> Payload<'a, C> for Option<T> {}

impl<T: PayloadInfo> PayloadInfo for Option<T> {
    const HASH: u64 = PayloadConstHash(stringify!(Option<T>).as_bytes());
    const TYPE: &'static str = "Option<T>";
    const SIZE: Option<usize> = size_add(T::SIZE, u8::SIZE);
}

impl<'a, C: PayloadContext, T: IntoPayload<C> + PayloadInfo, E: IntoPayload<C> + PayloadInfo> IntoPayload<C> for Result<T, E> {
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        match self {
            Ok(res) => {
                next.into_payload(&1u8, handler, ctx)?;
                next.into_payload(res, handler, ctx)
            },
            Err(error) => {
                next.into_payload(&0u8, handler, ctx)?;
                next.into_payload(error, handler, ctx)
            }
        }
    }
}

impl<'a, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo, E: FromPayload<'a, C> + PayloadInfo> FromPayload<'a, C> for Result<T, E> {
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b
    {
        let byte: u8 = next.from_payload(handler, ctx)?;

        if byte != 0 {
            let res: T = next.from_payload(handler, ctx)?;

            Ok(Ok(res))
        } else {
            let res: E = next.from_payload(handler, ctx)?;

            Ok(Err(res))
        }
    }
}

impl<'a, C: PayloadContext, T: Payload<'a, C> + PayloadInfo, E: Payload<'a, C> + PayloadInfo> Payload<'a, C> for Result<T, E> {}

impl<T: PayloadInfo, E: PayloadInfo> PayloadInfo for Result<T, E> {
    const HASH: u64 = PayloadConstHash(stringify!(Result<T, E>).as_bytes()) ^ T::HASH ^ E::HASH;
    const TYPE: &'static str = "Result<T, E>";
    const SIZE: Option<usize> = size_max(T::SIZE, E::SIZE);
}
