use core::mem;

use super::{Error, Middleware, Payload, IntoPayload, FromPayload};

impl<'a, C, T: IntoPayload<C>> IntoPayload<C> for &'a [T] {
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        if mem::size_of::<T>() == 1 {
            next.into_payload(&self.len(), ctx)?;
            next.write(self)?;
        } else {
            next.into_payload(&self.len(), ctx)?;

            for elem in *self {
                next.into_payload(elem, ctx)?;
            }
        }

        Ok(())
    }
}

impl<'a, C, T: FromPayload<'a, C>> FromPayload<'a, C> for &'a [T] {
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error> 
        where 'a: 'b,
    {
        let len: usize = next.from_payload(ctx)?;

        if mem::size_of::<T>() == 1 {
            next.read(len)
        } else {
            let mut vec = Vec::with_capacity(len);

            for _ in 0..len {
                vec.push(next.from_payload::<C, T>(ctx)?);
            }

            // TODO(): Replace Box::leak()
            Ok(Box::leak(vec.into_boxed_slice()))
        }
    }
}

impl<'a, C, T: Payload<C>> Payload<C> for &'a [T] {}

impl<'a, C, T: IntoPayload<C>> IntoPayload<C> for &mut [T] {
    #[inline]
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload::<C, &[T]>(&self.as_ref(), ctx)
    }
}

impl<'a, C, T: FromPayload<'a, C>> FromPayload<'a, C> for &'a mut [T] where T: Clone {
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        let len: usize = next.from_payload(ctx)?;

        if mem::size_of::<T>() == 1 {
            next.read_mut(len)
        } else {
            let mut vec = Vec::with_capacity(len);

            for _ in 0..len {
                vec.push(next.from_payload::<C, T>(ctx)?);
            }

            // TODO(): Replace Box::leak()
            Ok(Box::leak(vec.into_boxed_slice()))
        }
    }
}

impl<'a, C, T: Payload<C>> Payload<C> for &'a mut [T] 
    where T: Clone {}

impl<C, T: IntoPayload<C>, const N: usize> IntoPayload<C> for [T; N] {
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        if mem::size_of::<T>() == 1 {
            next.write(self)?;
        } else {
            for elem in self.into_iter() {
                next.into_payload(elem, ctx)?;
            }
        }
        
        Ok(())
    }
}

impl<'a, C, T: FromPayload<'a, C> + 'a, const N: usize> FromPayload<'a, C> for [T; N] 
    where T: Copy
{
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        if mem::size_of::<T>() == 1 {
            let bytes: &[T] = next.read(N)?;

            Ok(unsafe {
                *(bytes.as_ptr() as *const [T; N])
            })
        } else {
            let mut vec = Vec::with_capacity(N);

            for _ in 0..N {
                vec.push(next.from_payload::<C, T>(ctx)?);
            }

            Ok(unsafe { *(vec.leak().as_ptr() as *const [T; N]) })
        }
    }
}

impl<C, T: Payload<C>, const N: usize> Payload<C> for [T; N] where T: Copy {}