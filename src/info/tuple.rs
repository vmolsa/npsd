use super::{PayloadInfo, helper::size_array};

#[macro_export]
macro_rules! payload_info_tuple {
    (($(($T:ident, $i:tt)),+), $len:tt) => {
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

payload_info_tuple!(((A, 0)), 1);
payload_info_tuple!(((A, 0), (B, 1)), 2);
payload_info_tuple!(((A, 0), (B, 1), (C, 2)), 3);
payload_info_tuple!(((A, 0), (B, 1), (C, 2), (D, 3)), 4);
payload_info_tuple!(((A, 0), (B, 1), (C, 2), (D, 3), (E, 4)), 5);
payload_info_tuple!(((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5)), 6);
payload_info_tuple!(((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6)), 7);
payload_info_tuple!(((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7)), 8);