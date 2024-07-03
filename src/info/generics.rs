use super::PayloadInfo;

impl<T: PayloadInfo> PayloadInfo for &T {
    const HASH: u64 = T::HASH;
    const TYPE: &'static str = T::TYPE;
    const SIZE: Option<usize> = T::SIZE;
}

impl<T: PayloadInfo> PayloadInfo for &mut T {
    const HASH: u64 = T::HASH;
    const TYPE: &'static str = T::TYPE;
    const SIZE: Option<usize> = T::SIZE;
}