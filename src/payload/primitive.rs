use core::{mem, str};

use super::{Error, Middleware, PayloadContext, PayloadHandler, PayloadInfo, Payload, IntoPayload, FromPayload};

#[macro_export]
macro_rules! payload_be_bytes {
    ($type:ty) => {
        impl<'a, C: PayloadContext> IntoPayload<C> for $type {
            #[inline]
            fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
                next.into_payload(&self.to_be_bytes(), handler, ctx)
            }
        }
        
        impl<'a, C: PayloadContext> FromPayload<'a, C> for $type {
            #[inline]
            fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
                where 'a: 'b,
            {
                Ok(<Self>::from_be_bytes(next.from_payload(handler, ctx)?))
            }
        }

        impl<'a, C: PayloadContext> Payload<'a, C> for $type {}

        impl PayloadInfo for $type {
            const TYPE: &'static str = stringify!($type);
            const SIZE: Option<usize> = Some(mem::size_of::<Self>());
        }
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

impl<'a, C: PayloadContext> IntoPayload<C> for isize {
    #[inline]
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&(*self as i64).to_be_bytes(), handler, ctx)
    }
}

impl<'a, C: PayloadContext> FromPayload<'a, C> for isize {
    #[inline]
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(<i64>::from_be_bytes(next.from_payload(handler, ctx)?) as Self)
    }
}

impl<'a, C: PayloadContext> Payload<'a, C> for isize {}

impl PayloadInfo for isize {
    const TYPE: &'static str = "isize";
    const SIZE: Option<usize> = Some(mem::size_of::<Self>());
}

impl<'a, C: PayloadContext> IntoPayload<C> for usize {
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        let mut value = *self;
        const CONTINUATION_BIT: u8 = 0b1000_0000;
        const DATA_BITS: usize = 7;

        loop {
            if value < 0x80 {
                next.into_payload(&(value as u8), handler, ctx)?;
                break;
            } else {
                next.into_payload(&(((value as u8) & 0x7F) | CONTINUATION_BIT), handler, ctx)?;

                value >>= DATA_BITS;
            }
        }

        Ok(())
    }
}

impl<'a, C: PayloadContext> FromPayload<'a, C> for usize {
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error> 
        where 'a: 'b,
    {
        const CONTINUATION_BIT: u8 = 0b1000_0000;
        const DATA_BITS: u8 = 7;

        let mut result = 0usize;
        let mut shift = 0u8;

        loop {
            let byte: u8 = next.from_payload(handler, ctx)?;

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

impl<'a, C: PayloadContext> Payload<'a, C> for usize {}

impl PayloadInfo for usize {
    const TYPE: &'static str = "usize";
    const SIZE: Option<usize> = Some(mem::size_of::<Self>());
}

impl<'a, C: PayloadContext> IntoPayload<C> for () {
    fn into_payload<'b, M: Middleware>(&'b self, _handler: &mut PayloadHandler<'_>, _ctx: &mut C, _next: &mut M) -> Result<(), Error> {
        Ok(())
    }
}

impl<'a, C: PayloadContext> FromPayload<'a, C> for () {
    fn from_payload<'b, M: Middleware>(_handler: &'b mut PayloadHandler<'_>, _ctx: &mut C, _next: &mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(())
    }
}

impl<'a, C: PayloadContext> Payload<'a, C> for () {}

impl PayloadInfo for () {
    const TYPE: &'static str = "()";
    const SIZE: Option<usize> = Some(mem::size_of::<Self>());
}

impl<'a, C: PayloadContext> IntoPayload<C> for bool {
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        if *self {
            next.into_payload(&1u8, handler, ctx)
        } else {
            next.into_payload(&0u8, handler, ctx)
        }
    }
}

impl<'a, C: PayloadContext> FromPayload<'a, C> for bool {
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        let byte: u8 = next.from_payload(handler, ctx)?;

        if byte != 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl<'a, C: PayloadContext> Payload<'a, C> for bool {}

impl PayloadInfo for bool {
    const TYPE: &'static str = "bool";
    const SIZE: Option<usize> = Some(mem::size_of::<Self>());
}
