use core::str;
use std::str::FromStr;

use super::{Error, Middleware, Payload, IntoPayload, FromPayload};

impl<C> IntoPayload<C> for char {
    #[inline]
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&(*self as u32).to_be_bytes(), ctx)
    }
}

impl<'a, C> FromPayload<'a, C> for char {
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b
    {
        let ch: u32 = next.from_payload(ctx)?;

        if let Some(ch) = char::from_u32(ch) {
            Ok(ch)
        } else {
            Err(Error::UnknownVariant("Exptected char from u32".to_string()))
        }
    }
}

impl<C> Payload<C> for char {}

impl<'a, C> IntoPayload<C> for &'a str {
    #[inline]
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&self.as_bytes(), ctx)
    }
}

impl<'a, C> FromPayload<'a, C> for &'a str {
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b
    {
        let nbytes: usize = next.from_payload(ctx)?;

        str::from_utf8(next.read(nbytes)?).map_err(|e| {
            Error::InvalidUtf8(e.to_string())
        })
    }
}

impl<'a, C> Payload<C> for &'a str {}

impl<'a, C> IntoPayload<C> for &mut str {
    #[inline]
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&self.as_bytes(), ctx)
    }
}

impl<'a, C> FromPayload<'a, C> for &'a mut str {
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b
    {
        let nbytes: usize = next.from_payload(ctx)?;
        
        str::from_utf8_mut(next.read_mut(nbytes)?).map_err(|e| {
            Error::InvalidUtf8(e.to_string())
        })
    }
}

impl<'a, C> Payload<C> for &'a mut str {}

impl<C> IntoPayload<C> for String {
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&self.as_bytes(), ctx)
    }
}

impl<'a, C> FromPayload<'a, C> for String {
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b
    {
        String::from_str(next.from_payload(ctx)?).map_err(|e| {
            Error::InvalidUtf8(e.to_string())
        })
    }
}

impl<C> Payload<C> for String {}
