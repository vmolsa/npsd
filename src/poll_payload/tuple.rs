use super::{Error, AsyncMiddleware, AsyncPayload, AsyncIntoPayload, AsyncFromPayload};

#[macro_export]
macro_rules! async_payload_tuple {
    (($(($T:ident, $i:tt)),+), $len:tt) => {
        impl<X: Send + Sync, $($T),+> AsyncIntoPayload<X> for ($($T,)+) 
            where
                $($T: AsyncIntoPayload<X> + Send + Sync,)+
        {
            async fn poll_into_payload<Y: AsyncMiddleware>(&self, ctx: &mut X, next: &mut Y) -> Result<(), Error> {
                $(
                    next.poll_into_payload(&self.$i, ctx).await?;
                )+

                Ok(())
            }
        }

        impl<'a, X: Send + Sync, $($T),+> AsyncFromPayload<'a, X> for ($($T,)+) 
            where
                $($T: AsyncFromPayload<'a, X>,)+
        {
            async fn poll_from_payload<'b, Y: AsyncMiddleware>(ctx: &mut X, next: &'b mut Y) -> Result<Self, Error>
                where 'a: 'b,
            {
                Ok(($(
                    next.poll_from_payload::<X, $T>(ctx).await?,
                )+))
            }
        }

        impl<X: Send + Sync, $($T),+> AsyncPayload<X> for ($($T,)+) 
            where
                $($T: AsyncPayload<X>,)+ {}
    };
}

async_payload_tuple!(((A, 0)), 1);
async_payload_tuple!(((A, 0), (B, 1)), 2);
async_payload_tuple!(((A, 0), (B, 1), (C, 2)), 3);
async_payload_tuple!(((A, 0), (B, 1), (C, 2), (D, 3)), 4);
async_payload_tuple!(((A, 0), (B, 1), (C, 2), (D, 3), (E, 4)), 5);
async_payload_tuple!(((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5)), 6);
async_payload_tuple!(((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6)), 7);
async_payload_tuple!(((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7)), 8);