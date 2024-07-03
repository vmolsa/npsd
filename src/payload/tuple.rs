use super::{Error, Payload, IntoPayload, FromPayload, Middleware};

#[macro_export]
macro_rules! payload_tuple {
    (($(($T:ident, $i:tt)),+), $len:tt) => {
        impl<X, $($T),+> IntoPayload<X> for ($($T,)+) 
            where
                $($T: IntoPayload<X>,)+
        {
            fn into_payload<Y: Middleware>(&self, ctx: &mut X, next: &mut Y) -> Result<(), Error> {
                $(
                    next.into_payload(&self.$i, ctx)?;
                )+

                Ok(())
            }
        }

        impl<'a, X, $($T),+> FromPayload<'a, X> for ($($T,)+) 
            where
                $($T: FromPayload<'a, X>,)+
        {
            fn from_payload<'b, Y: Middleware>(ctx: &mut X, next: &'b mut Y) -> Result<Self, Error>
                where 'a: 'b,
            {
                Ok(($(
                    next.from_payload::<X, $T>(ctx)?,
                )+))
            }
        }

        impl<X, $($T),+> Payload<X> for ($($T,)+) 
            where
                $($T: Payload<X>,)+ {}
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