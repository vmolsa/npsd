use super::{Error, Middleware, PayloadContext, PayloadHandler, PayloadInfo, Payload, IntoPayload, FromPayload};

impl<'a, C: PayloadContext, T: IntoPayload<C> + PayloadInfo> IntoPayload<C> for *mut T {
    #[inline]
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        unsafe {
            if self.is_null() {
                return Err(Error::NullPtr);
            }

            next.into_payload(&**self, handler, ctx)
        }
    }
}

impl<'a, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo> FromPayload<'a, C> for *mut T {
    #[inline]
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        let value = next.from_payload::<C, T>(handler, ctx)?;
        let boxed = Box::new(value);
        
        Ok(Box::into_raw(boxed))
    }
}

impl<'a, C: PayloadContext, T: Payload<'a, C> + PayloadInfo> Payload<'a, C> for *mut T {}

impl<T: PayloadInfo> PayloadInfo for *mut T {
    const HASH: u64 = T::HASH;
    const TYPE: &'static str = T::TYPE;
    const SIZE: Option<usize> = T::SIZE;
}