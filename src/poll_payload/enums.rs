use super::{Error, AsyncFromPayload, AsyncIntoPayload, AsyncMiddleware, AsyncPayload};

impl<C, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for Option<T> {
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
        if let Some(data) = self {
            next.poll_into_payload(&1u8, ctx).await?;
            next.poll_into_payload(data, ctx).await
        } else {
            next.poll_into_payload(&0u8, ctx).await
        }
    }
}

impl<'a, C, T: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for Option<T> {
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b
    {
        let byte: u8 = next.poll_from_payload(ctx).await?;

        if byte != 0 {
            let res: T = next.poll_from_payload(ctx).await?;

            Ok(Some(res))
        } else {
            Ok(None)
        }
    }
}

impl<C, T: AsyncPayload<C>> AsyncPayload<C> for Option<T> {}

impl<C, T: AsyncIntoPayload<C>, E: AsyncIntoPayload<C>> AsyncIntoPayload<C> for Result<T, E> {
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
        match self {
            Ok(res) => {
                next.poll_into_payload(&1u8, ctx).await?;
                next.poll_into_payload(res, ctx).await
            },
            Err(error) => {
                next.poll_into_payload(&0u8, ctx).await?;
                next.poll_into_payload(error, ctx).await
            }
        }
    }
}

impl<'a, C, T: AsyncFromPayload<'a, C>, E: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for Result<T, E> {
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b
    {
        let byte: u8 = next.poll_from_payload(ctx).await?;

        if byte != 0 {
            let res: T = next.poll_from_payload(ctx).await?;

            Ok(Ok(res))
        } else {
            let res: E = next.poll_from_payload(ctx).await?;

            Ok(Err(res))
        }
    }
}

impl<C, T: AsyncPayload<C>, E: AsyncPayload<C>> AsyncPayload<C> for Result<T, E> {}
