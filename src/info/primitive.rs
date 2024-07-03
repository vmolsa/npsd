use core::{mem, str};

use super::PayloadInfo;

impl PayloadInfo for u8 {
    const TYPE: &'static str = stringify!(u8);
    const SIZE: Option<usize> = Some(mem::size_of::<Self>());
}

impl PayloadInfo for i8 {
    const TYPE: &'static str = stringify!(i8);
    const SIZE: Option<usize> = Some(mem::size_of::<Self>());
}

impl PayloadInfo for u16 {
    const TYPE: &'static str = stringify!(u16);
    const SIZE: Option<usize> = Some(mem::size_of::<Self>());
}

impl PayloadInfo for i16 {
    const TYPE: &'static str = stringify!(i16);
    const SIZE: Option<usize> = Some(mem::size_of::<Self>());
}

impl PayloadInfo for u32 {
    const TYPE: &'static str = stringify!(u32);
    const SIZE: Option<usize> = Some(mem::size_of::<Self>());
}

impl PayloadInfo for i32 {
    const TYPE: &'static str = stringify!(i32);
    const SIZE: Option<usize> = Some(mem::size_of::<Self>());
}

impl PayloadInfo for u64 {
    const TYPE: &'static str = stringify!(u64);
    const SIZE: Option<usize> = Some(mem::size_of::<Self>());
}

impl PayloadInfo for i64 {
    const TYPE: &'static str = stringify!(i64);
    const SIZE: Option<usize> = Some(mem::size_of::<Self>());
}

impl PayloadInfo for u128 {
    const TYPE: &'static str = stringify!(u128);
    const SIZE: Option<usize> = Some(mem::size_of::<Self>());
}

impl PayloadInfo for i128 {
    const TYPE: &'static str = stringify!(i128);
    const SIZE: Option<usize> = Some(mem::size_of::<Self>());
}

impl PayloadInfo for f32 {
    const TYPE: &'static str = stringify!(f32);
    const SIZE: Option<usize> = Some(mem::size_of::<Self>());
}

impl PayloadInfo for f64 {
    const TYPE: &'static str = stringify!(f64);
    const SIZE: Option<usize> = Some(mem::size_of::<Self>());
}

impl PayloadInfo for isize {
    const TYPE: &'static str = "isize";
    const SIZE: Option<usize> = Some(mem::size_of::<Self>());
}

impl PayloadInfo for usize {
    const TYPE: &'static str = "usize";
    const SIZE: Option<usize> = Some(mem::size_of::<Self>());
}

impl PayloadInfo for () {
    const TYPE: &'static str = "()";
    const SIZE: Option<usize> = Some(mem::size_of::<Self>());
}

impl PayloadInfo for bool {
    const TYPE: &'static str = "bool";
    const SIZE: Option<usize> = Some(mem::size_of::<Self>());
}