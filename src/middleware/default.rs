use crate::{Error, FromPayload, IntoPayload, Middleware, PayloadContext, PayloadHandler, PayloadInfo};

/// A no-op implementation of the `Middleware` trait.
///
/// This implementation is used when no middleware processing is required. It simply forwards the
/// serialization and deserialization tasks to the respective methods of the value being processed.
///
/// # Methods
/// - `fn into_payload<C: PayloadContext, T: IntoPayload<C> + PayloadInfo>(
///       &mut self,
///       value: &T,
///       handler: &mut PayloadHandler<'_>,
///       ctx: &mut C
///   ) -> Result<(), Error>`:
///     - Converts a value into a payload of bytes. This method forwards the serialization task to
///       the `into_payload` method of the value being processed.
/// - `fn from_payload<'a, 'b, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo>(
///       &mut self,
///       handler: &'b mut PayloadHandler<'a>,
///       ctx: &mut C
///   ) -> Result<T, Error>`:
///     - Converts a payload of bytes back into a value. This method forwards the deserialization task
///       to the `from_payload` method of the type being processed.
impl Middleware for () {
    #[inline(always)]
    fn into_payload<C: PayloadContext, T: IntoPayload<C> + PayloadInfo>(&mut self, value: &T, handler: &mut PayloadHandler<'_>, ctx: &mut C) -> Result<(), Error> {   
        value.into_payload(handler, ctx, &mut ())
    }

    #[inline(always)]
    fn from_payload<'a, 'b, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo>(&mut self, handler: &'b mut PayloadHandler<'a>, ctx: &mut C) -> Result<T, Error>
        where
            'a: 'b 
    {
        T::from_payload(handler, ctx, &mut ())
    }
}

