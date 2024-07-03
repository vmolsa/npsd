use core::mem;

use super::{Error, Middleware, Payload, IntoPayload, FromPayload};

#[macro_export]
macro_rules! payload_be_bytes {
    ($type:ty) => {
        impl<C> IntoPayload<C> for $type {
            #[inline]
            fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
                next.into_payload(&self.to_be_bytes(), ctx)
            }
        }
        
        impl<'a, C> FromPayload<'a, C> for $type {
            #[inline]
            fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
                where 'a: 'b,
            {
                Ok(<Self>::from_be_bytes(next.from_payload(ctx)?))
            }
        }

        impl<C> Payload<C> for $type {}
    };
}

payload_be_bytes!(i8);
payload_be_bytes!(u8);
payload_be_bytes!(i16);
payload_be_bytes!(u16);
payload_be_bytes!(i32);
payload_be_bytes!(u32);
payload_be_bytes!(i64);
payload_be_bytes!(u64);
payload_be_bytes!(i128);
payload_be_bytes!(u128);
payload_be_bytes!(f32);
payload_be_bytes!(f64);

impl<C> IntoPayload<C> for isize {
    #[inline]
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&(*self as i64).to_be_bytes(), ctx)
    }
}

impl<'a, C> FromPayload<'a, C> for isize {
    #[inline]
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(<i64>::from_be_bytes(next.from_payload(ctx)?) as Self)
    }
}

impl<C> Payload<C> for isize {}

impl<C> IntoPayload<C> for usize {
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        let mut value = *self;
        const CONTINUATION_BIT: u8 = 0b1000_0000;
        const DATA_BITS: usize = 7;

        loop {
            if value < 0x80 {
                next.into_payload(&(value as u8), ctx)?;
                break;
            } else {
                next.into_payload(&(((value as u8) & 0x7F) | CONTINUATION_BIT), ctx)?;

                value >>= DATA_BITS;
            }
        }

        Ok(())
    }
}

impl<'a, C> FromPayload<'a, C> for usize {
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error> 
        where 'a: 'b,
    {
        const CONTINUATION_BIT: u8 = 0b1000_0000;
        const DATA_BITS: u8 = 7;

        let mut result = 0usize;
        let mut shift = 0u8;

        loop {
            let byte: u8 = next.from_payload(ctx)?;

            let value = (byte & !CONTINUATION_BIT) as usize;
            result |= value << usize::from(shift);
            
            if byte & CONTINUATION_BIT == 0 {
                break;
            }

            shift += DATA_BITS;

            if usize::from(shift) >= mem::size_of::<usize>() * 8 {
                return Err(Error::IndexOutOfBounds(usize::from(shift)));
            }
        }

        Ok(result)
    }
}

impl<C> Payload<C> for usize {}

impl<C> IntoPayload<C> for () {
    fn into_payload<M: Middleware>(&self, _ctx: &mut C, _next: &mut M) -> Result<(), Error> {
        Ok(())
    }
}

impl<'a, C> FromPayload<'a, C> for () {
    fn from_payload<'b, M: Middleware>(_ctx: &mut C, _next: &mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(())
    }
}

impl<C> Payload<C> for () {}

impl<C> IntoPayload<C> for bool {
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        if *self {
            next.into_payload(&1u8, ctx)
        } else {
            next.into_payload(&0u8, ctx)
        }
    }
}

impl<'a, C> FromPayload<'a, C> for bool {
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        let byte: u8 = next.from_payload(ctx)?;

        if byte != 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl<C> Payload<C> for bool {}

