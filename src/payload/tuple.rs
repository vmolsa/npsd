use super::{Error, PayloadContext, PayloadHandler, PayloadInfo, Payload, IntoPayload, FromPayload, Middleware, helper::size_array};

#[macro_export]
macro_rules! payload_tuple {
    (($(($T:ident, $i:tt)),+), $len:tt) => {
        impl<X: PayloadContext, $($T),+> IntoPayload<X> for ($($T,)+) 
            where
                $($T: IntoPayload<X> + PayloadInfo,)+
        {
            fn into_payload<'b, Y: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut X, next: &mut Y) -> Result<(), Error> {
                $(
                    next.into_payload(&self.$i, handler, ctx)?;
                )+

                Ok(())
            }
        }

        impl<'a, X: PayloadContext, $($T),+> FromPayload<'a, X> for ($($T,)+) 
            where
                $($T: FromPayload<'a, X> + PayloadInfo,)+
        {
            fn from_payload<'b, Y: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut X, next: &'b mut Y) -> Result<Self, Error>
                where 'a: 'b,
            {
                Ok(($(
                    next.from_payload::<X, $T>(handler, ctx)?,
                )+))
            }
        }

        impl<'a, X: PayloadContext, $($T),+> Payload<'a, X> for ($($T,)+) 
            where
                $($T: Payload<'a, X> + PayloadInfo,)+ {}

        impl<'a, $($T),+> PayloadInfo for ($($T,)+) 
            where
                $($T: PayloadInfo,)+
        {
            const HASH: u64 = 0 $( ^ <$T>::HASH )+;
            const TYPE: &'static str = "tuple";
            const SIZE: Option<usize> = size_array([$(
                <$T>::SIZE
            ),+]);
        }
    };
}

payload_tuple!(((A, 0)), 1);
payload_tuple!(((A, 0), (B, 1)), 2);
payload_tuple!(((A, 0), (B, 1), (C, 2)), 3);
payload_tuple!(((A, 0), (B, 1), (C, 2), (D, 3)), 4);
payload_tuple!(((A, 0), (B, 1), (C, 2), (D, 3), (E, 4)), 5);
payload_tuple!(((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5)), 6);
payload_tuple!(((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6)), 7);
payload_tuple!(((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7)), 8);