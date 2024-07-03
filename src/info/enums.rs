use super::{size_add, size_max, PayloadConstHash, PayloadInfo};

impl<T: PayloadInfo> PayloadInfo for Option<T> {
    const HASH: u64 = PayloadConstHash(stringify!(Option<T>).as_bytes());
    const TYPE: &'static str = "Option<T>";
    const SIZE: Option<usize> = size_add(T::SIZE, u8::SIZE);
}

impl<T: PayloadInfo, E: PayloadInfo> PayloadInfo for Result<T, E> {
    const HASH: u64 = PayloadConstHash(stringify!(Result<T, E>).as_bytes()) ^ T::HASH ^ E::HASH;
    const TYPE: &'static str = "Result<T, E>";
    const SIZE: Option<usize> = size_max(T::SIZE, E::SIZE);
}
