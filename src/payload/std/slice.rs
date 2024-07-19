use core::mem;

use crate::AnyBox;

use super::{Error, Middleware, Payload, IntoPayload, FromPayload};

impl<'a, C, T: IntoPayload<C>> IntoPayload<C> for &'a [T] {
    fn into_payload<'m, M: Middleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
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

impl<'a, C, T: FromPayload<'a, C> + AnyBox<'a>> FromPayload<'a, C> for &'a [T] {
    fn from_payload<M: Middleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let len: usize = next.from_payload(ctx)?;

        if mem::size_of::<T>() == 1 {
            next.read(len)
        } else {
            let mut vec = Vec::with_capacity(len);

            for _ in 0..len {
                vec.push(next.from_payload::<C, T>(ctx)?);
            }

            next.push_array(vec.into_boxed_slice())
        }
    }
}

impl<'a, C, T: Payload<'a, C> + AnyBox<'a>> Payload<'a, C> for &'a [T] {}

impl<'a, C, T: IntoPayload<C>> IntoPayload<C> for &mut [T] {
    #[inline]
    fn into_payload<'m, M: Middleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload::<C, &[T]>(&self.as_ref(), ctx)
    }
}

impl<'a, C, T: FromPayload<'a, C> + AnyBox<'a>> FromPayload<'a, C> for &'a mut [T] where T: Clone {
    fn from_payload<M: Middleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        if mem::size_of::<T>() == 1 {
            let nbytes: usize = next.from_payload(ctx)?;

            next.read_mut(nbytes)
        } else {
            let vec: Vec<T> = next.from_payload(ctx)?;

            next.push_array_mut(vec.into_boxed_slice())
        }
    }
}

impl<'a, C, T: Payload<'a, C> + AnyBox<'a>> Payload<'a, C> for &'a mut [T] 
    where T: Clone {}

impl<'a, C, T: IntoPayload<C>, const N: usize> IntoPayload<C> for [T; N] {
    fn into_payload<'m, M: Middleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
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

impl<'a, C, T: FromPayload<'a, C> + AnyBox<'a> + 'a, const N: usize> FromPayload<'a, C> for [T; N] 
    where T: Copy
{
    fn from_payload<M: Middleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
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

            let slice = next.push_array(vec.into_boxed_slice())?;

            Ok(unsafe { *(slice.as_ptr() as *const [T; N]) })
        }
    }
}

impl<'a, C, T: Payload<'a, C> + AnyBox<'a>, const N: usize> Payload<'a, C> for [T; N] where T: Copy {}