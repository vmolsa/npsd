// use super::{Error, AsyncMiddleware, AsyncPayload, AsyncIntoPayload, AsyncFromPayload};

// TODO(): Disabled because Send + Sync
// impl<C: Send + Sync, T: AsyncIntoPayload<C> + Send> AsyncIntoPayload<C> for *mut T {
//     #[inline]
//     async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
//         unsafe {
//             if self.is_null() {
//                 return Err(Error::NullPtr);
//             }

//             next.poll_into_payload(&**self, ctx).await
//         }
//     }
// }

// impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C> + Send> AsyncFromPayload<'a, C> for *mut T {
//     #[inline]
//     async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error>
//         where 'a: 'b,
//     {
//         let value = next.poll_from_payload::<C, T>(ctx).await?;
//         let boxed = Box::new(value);
        
//         Ok(Box::into_raw(boxed))
//     }
// }

// impl<C: Send + Sync, T: AsyncPayload<'a, C> + Send> AsyncPayload<'a, C> for *mut T {}
