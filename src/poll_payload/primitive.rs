use core::mem;

use super::{Error, AsyncMiddleware, AsyncPayload, AsyncIntoPayload, AsyncFromPayload};

#[macro_export]
macro_rules! async_payload_be_bytes {
    ($type:ty) => {
        impl<C: Send + Sync> AsyncIntoPayload<C> for $type {
            #[inline]
            async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, _ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
                next.poll_write(&self.to_be_bytes()).await
            }
        }
        
        impl<'a, C: Send + Sync> AsyncFromPayload<'a, C> for $type {
            #[inline]
            async fn poll_from_payload<'b, M: AsyncMiddleware>(_ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
                where 'a: 'b,
            {
                let slice: &[u8] = next.poll_read(mem::size_of::<Self>()).await?;

                Ok(<Self>::from_be_bytes(unsafe {
                    *(slice.as_ptr() as *const [u8; mem::size_of::<Self>()])
                }))
            }
        }

        impl<C: Send + Sync> AsyncPayload<C> for $type {}
    };
}

async_payload_be_bytes!(i8);
async_payload_be_bytes!(u8);
async_payload_be_bytes!(i16);
async_payload_be_bytes!(u16);
async_payload_be_bytes!(i32);
async_payload_be_bytes!(u32);
async_payload_be_bytes!(i64);
async_payload_be_bytes!(u64);
async_payload_be_bytes!(i128);
async_payload_be_bytes!(u128);
async_payload_be_bytes!(f32);
async_payload_be_bytes!(f64);

impl<C: Send + Sync> AsyncIntoPayload<C> for isize {
    #[inline]
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
        next.poll_into_payload(&(*self as i64), ctx).await
    }
}

impl<'a, C: Send + Sync> AsyncFromPayload<'a, C> for isize {
    #[inline]
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        let value: i64 = next.poll_from_payload(ctx).await?;
        Ok(value as Self)
    }
}

impl<C: Send + Sync> AsyncPayload<C> for isize {}

impl<C: Send + Sync> AsyncIntoPayload<C> for usize {
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
        let mut value = *self;
        const CONTINUATION_BIT: u8 = 0b1000_0000;
        const DATA_BITS: usize = 7;

        loop {
            if value < 0x80 {
                next.poll_into_payload(&(value as u8), ctx).await?;
                break;
            } else {
                next.poll_into_payload(&(((value as u8) & 0x7F) | CONTINUATION_BIT), ctx).await?;

                value >>= DATA_BITS;
            }
        }

        Ok(())
    }
}

impl<'a, C: Send + Sync> AsyncFromPayload<'a, C> for usize {
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error> 
        where 'a: 'b,
    {
        const CONTINUATION_BIT: u8 = 0b1000_0000;
        const DATA_BITS: u8 = 7;

        let mut result = 0usize;
        let mut shift = 0u8;

        loop {
            let byte: u8 = next.poll_from_payload(ctx).await?;

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

impl<C: Send + Sync> AsyncPayload<C> for usize {}

impl<C: Send + Sync> AsyncIntoPayload<C> for () {
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, _ctx: &mut C, _next: &'b mut M) -> Result<(), Error> {
        Ok(())
    }
}

impl<'a, C: Send + Sync> AsyncFromPayload<'a, C> for () {
    async fn poll_from_payload<'b, M: AsyncMiddleware>(_ctx: &mut C, _next: &'b mut M) -> Result<Self, Error> 
        where
            'a: 'b
    {
        Ok(())
    }
}

impl<C: Send + Sync> AsyncPayload<C> for () {}

impl<C: Send + Sync> AsyncIntoPayload<C> for bool {
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
        if *self {
            next.poll_into_payload(&1u8, ctx).await
        } else {
            next.poll_into_payload(&0u8, ctx).await
        }
    }
}

impl<'a, C: Send + Sync> AsyncFromPayload<'a, C> for bool {
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        let byte: u8 = next.poll_from_payload(ctx).await?;

        if byte != 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl<C: Send + Sync> AsyncPayload<C> for bool {}
