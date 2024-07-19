use super::{Error, Middleware, Payload, IntoPayload, FromPayload};

impl<C, T: IntoPayload<C>> IntoPayload<C> for *mut T {
    fn into_payload<'m, M: Middleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        unsafe {
            if self.is_null() {
                return Err(Error::NullPtr);
            }

            next.into_payload(&**self, ctx)
        }
    }
}

impl<'a, C, T: FromPayload<'a, C>> FromPayload<'a, C> for *mut T {
    fn from_payload<M: Middleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let value = next.from_payload::<C, T>(ctx)?;
        let boxed = Box::new(value);
        
        Ok(Box::into_raw(boxed))
    }
}

impl<'a, C, T: Payload<'a, C>> Payload<'a, C> for *mut T {}