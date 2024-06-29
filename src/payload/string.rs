use core::{mem, str};

use super::{Error, Middleware, PayloadContext, PayloadHandler, PayloadInfo, Payload, IntoPayload, FromPayload};

impl<C: PayloadContext> IntoPayload<C> for char {
    #[inline]
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&(*self as u32).to_be_bytes(), handler, ctx)
    }
}

impl<'a, C: PayloadContext> FromPayload<'a, C> for char {
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b
    {
        let ch: u32 = next.from_payload(handler, ctx)?;

        if let Some(ch) = char::from_u32(ch) {
            Ok(ch)
        } else {
            Err(Error::UnknownVariant("Exptected char from u32".to_string()))
        }
    }
}

impl<'a, C: PayloadContext> Payload<'a, C> for char {}

impl PayloadInfo for char {
    const TYPE: &'static str = "char";
    const SIZE: Option<usize> = Some(mem::size_of::<Self>());
}

impl<'a, C: PayloadContext> IntoPayload<C> for &'a str {
    #[inline]
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&self.as_bytes(), handler, ctx)
    }
}

impl<'a, C: PayloadContext> FromPayload<'a, C> for &'a str {
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b
    {
        let slice: &[u8] = next.from_payload(handler, ctx)?;

        let result = str::from_utf8(slice).map_err(|e| {
            Error::InvalidUtf8(e.to_string())
        })?;

        Ok(result)
    }
}

impl<'a, C: PayloadContext> Payload<'a, C> for &'a str {}

impl<'a> PayloadInfo for &'a str {
    const TYPE: &'static str = "&str";
}

impl<'a, C: PayloadContext> IntoPayload<C> for &mut str {
    #[inline]
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&self.as_bytes(), handler, ctx)
    }
}

impl<'a, C: PayloadContext> FromPayload<'a, C> for &'a mut str {
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b
    {
        let slice: &mut [u8] = next.from_payload(handler, ctx)?;
        
        let result = str::from_utf8_mut(slice).map_err(|e| {
            Error::InvalidUtf8(e.to_string())
        })?;

        Ok(result)
    }
}

impl<'a, C: PayloadContext> Payload<'a, C> for &'a mut str {}

impl<'a> PayloadInfo for &'a mut str {
    const TYPE: &'static str = "&mut str";
}

impl<'a, C: PayloadContext> IntoPayload<C> for String {
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&self.as_bytes(), handler, ctx)
    }
}

impl<'a, C: PayloadContext> FromPayload<'a, C> for String {
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b
    {
        let slice: &mut str = next.from_payload(handler, ctx)?;

        let result = unsafe {
            String::from_raw_parts(slice.as_mut_ptr(), slice.len(), slice.len())
        };

        Ok(result)
    }
}

impl<'a, C: PayloadContext> Payload<'a, C> for String {}

impl PayloadInfo for String {
    const TYPE: &'static str = "String";
}
