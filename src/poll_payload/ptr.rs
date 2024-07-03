use super::{Error, AsyncMiddleware, AsyncPayload, AsyncIntoPayload, AsyncFromPayload};

impl<C, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for *mut T {
    #[inline]
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
        unsafe {
            if self.is_null() {
                return Err(Error::NullPtr);
            }

            next.poll_into_payload(&**self, ctx).await
        }
    }
}

impl<'a, C, T: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for *mut T {
    #[inline]
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        let value = next.poll_from_payload::<C, T>(ctx).await?;
        let boxed = Box::new(value);
        
        Ok(Box::into_raw(boxed))
    }
}

impl<C, T: AsyncPayload<C>> AsyncPayload<C> for *mut T {}
