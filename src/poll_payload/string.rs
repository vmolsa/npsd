use core::str;

use super::{Error, AsyncMiddleware, AsyncPayload, AsyncIntoPayload, AsyncFromPayload};

impl<C: Send + Sync> AsyncIntoPayload<C> for char {
    #[inline]
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
        next.poll_into_payload(&(*self as u32).to_be_bytes(), ctx).await
    }
}

impl<'a, C: Send + Sync> AsyncFromPayload<'a, C> for char {
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b
    {
        let ch: u32 = next.poll_from_payload(ctx).await?;

        if let Some(ch) = char::from_u32(ch) {
            Ok(ch)
        } else {
            Err(Error::UnknownVariant("Exptected char from u32".to_string()))
        }
    }
}

impl<C: Send + Sync> AsyncPayload<C> for char {}

impl<'a, C: Send + Sync> AsyncIntoPayload<C> for &'a str {
    #[inline]
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
        next.poll_into_payload(&self.as_bytes(), ctx).await
    }
}

impl<'a, C: Send + Sync> AsyncFromPayload<'a, C> for &'a str {
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b
    {
        let slice: &[u8] = next.poll_from_payload(ctx).await?;

        let result = str::from_utf8(slice).map_err(|e| {
            Error::InvalidUtf8(e.to_string())
        })?;

        Ok(result)
    }
}

impl<'a, C: Send + Sync> AsyncPayload<C> for &'a str {}

impl<'a, C: Send + Sync> AsyncIntoPayload<C> for &'a mut str {
    #[inline]
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
        next.poll_into_payload(&self.as_bytes(), ctx).await
    }
}

impl<'a, C: Send + Sync> AsyncFromPayload<'a, C> for &'a mut str {
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b
    {
        let slice: &mut [u8] = next.poll_from_payload(ctx).await?;
        
        let result = str::from_utf8_mut(slice).map_err(|e| {
            Error::InvalidUtf8(e.to_string())
        })?;

        Ok(result)
    }
}

impl<'a, C: Send + Sync> AsyncPayload<C> for &'a mut str {}

impl<C: Send + Sync> AsyncIntoPayload<C> for String {
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
        next.poll_into_payload(&self.as_bytes(), ctx).await
    }
}

impl<'a, C: Send + Sync> AsyncFromPayload<'a, C> for String {
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b
    {
        let slice: &mut str = next.poll_from_payload(ctx).await?;

        let result = unsafe {
            String::from_raw_parts(slice.as_mut_ptr(), slice.len(), slice.len())
        };

        Ok(result)
    }
}

impl<C: Send + Sync> AsyncPayload<C> for String {}
