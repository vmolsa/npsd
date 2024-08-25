
use std::ops::Range;

use super::{PayloadInfo, PayloadConstHash, size_mul};

impl<'a, T: PayloadInfo> PayloadInfo for &'a [T] {
    const HASH: u64 = PayloadConstHash(stringify!(&[T]).as_bytes()) ^ T::HASH;
    const TYPE: &'static str = "&[T]";
}

impl<'a, T: PayloadInfo> PayloadInfo for &'a mut [T] {
    const HASH: u64 = PayloadConstHash(stringify!(&[T]).as_bytes()) ^ T::HASH;
    const TYPE: &'static str = "&mut [T]";
}

impl<T: PayloadInfo, const N: usize> PayloadInfo for [T; N]  {
    const HASH: u64 = PayloadConstHash(stringify!(&[T]).as_bytes()) ^ N as u64 ^ T::HASH;
    const TYPE: &'static str = "[T; N] ";
    const SIZE: Option<usize> = size_mul(T::SIZE, N);
}

impl<T: PayloadInfo> PayloadInfo for Range<T> {
    const HASH: u64 = PayloadConstHash(stringify!(Range<T>).as_bytes()) ^ T::HASH;
    const TYPE: &'static str = "Range<T>";
}