use std::{cell::{Cell, Ref, RefCell, UnsafeCell}, pin::Pin, rc::Rc, sync::{Arc, Weak}};

use super::{PayloadInfo, PayloadConstHash};

impl<T: PayloadInfo> PayloadInfo for Box<T> {
    const HASH: u64 = T::HASH;
    const TYPE: &'static str = "Box<T>";
    const SIZE: Option<usize> = T::SIZE;
}

impl<T: PayloadInfo> PayloadInfo for Box<[T]> {
    const HASH: u64 = PayloadConstHash(stringify!(&[T]).as_bytes()) ^ T::HASH;
    const TYPE: &'static str = "Box<[T]>";
}

impl<T: PayloadInfo> PayloadInfo for Arc<T> {
    const HASH: u64 = T::HASH;
    const TYPE: &'static str = "Arc<T>";
    const SIZE: Option<usize> = T::SIZE;
}

impl<T: PayloadInfo> PayloadInfo for Arc<[T]> {
    const HASH: u64 = PayloadConstHash(stringify!(&[T]).as_bytes()) ^ T::HASH;
    const TYPE: &'static str = "Arc<[T]>";
}

impl<T: PayloadInfo> PayloadInfo for Rc<T> {
    const HASH: u64 = T::HASH;
    const TYPE: &'static str = "Rc<T>";
    const SIZE: Option<usize> = T::SIZE;
}

impl<T: PayloadInfo> PayloadInfo for Rc<[T]> {
    const HASH: u64 = PayloadConstHash(stringify!(&[T]).as_bytes()) ^ T::HASH;
    const TYPE: &'static str = "Rc<[T]>";
}

impl<T: PayloadInfo> PayloadInfo for UnsafeCell<T> {
    const HASH: u64 = T::HASH;
    const TYPE: &'static str = T::TYPE;
    const SIZE: Option<usize> = T::SIZE;
}

impl<T: PayloadInfo> PayloadInfo for Cell<T> {
    const HASH: u64 = T::HASH;
    const TYPE: &'static str = T::TYPE;
    const SIZE: Option<usize> = T::SIZE;
}

impl<T: PayloadInfo> PayloadInfo for Ref<'_, T> {
    const HASH: u64 = T::HASH;
    const TYPE: &'static str = "Ref<T>";
    const SIZE: Option<usize> = T::SIZE;
}

impl<T: PayloadInfo> PayloadInfo for RefCell<T> {
    const HASH: u64 = T::HASH;
    const TYPE: &'static str = T::TYPE;
    const SIZE: Option<usize> = T::SIZE;
}

impl<T: PayloadInfo> PayloadInfo for Pin<Box<T>> {
    const HASH: u64 = T::HASH;
    const TYPE: &'static str = T::TYPE;
    const SIZE: Option<usize> = T::SIZE;
}

impl<T: PayloadInfo> PayloadInfo for Weak<T> {
    const HASH: u64 = T::HASH;
    const TYPE: &'static str = T::TYPE;
    const SIZE: Option<usize> = T::SIZE;
}