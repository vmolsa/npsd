use super::{Error, Middleware, Payload, IntoPayload, FromPayload};

impl<C, T: IntoPayload<C>> IntoPayload<C> for *mut T {
    #[inline]
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        unsafe {
            if self.is_null() {
                return Err(Error::NullPtr);
            }

            next.into_payload(&**self, ctx)
        }
    }
}

impl<'a, C, T: FromPayload<'a, C>> FromPayload<'a, C> for *mut T {
    #[inline]
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        let value = next.from_payload::<C, T>(ctx)?;
        let boxed = Box::new(value);
        
        Ok(Box::into_raw(boxed))
    }
}

impl<C, T: Payload<C>> Payload<C> for *mut T {}