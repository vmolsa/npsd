use std::{borrow::Cow, collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque}};
use std::collections::{HashMap, HashSet};

use super::{PayloadInfo, PayloadConstHash};

impl<T: PayloadInfo> PayloadInfo for VecDeque<T> {
    const HASH: u64 = PayloadConstHash(stringify!(VecDeque<T>).as_bytes()) ^ T::HASH;
    const TYPE: &'static str = "VecDeque<T>";
    const SIZE: Option<usize> = None;
}

impl<T: PayloadInfo> PayloadInfo for LinkedList<T> {
    const HASH: u64 = PayloadConstHash(stringify!(LinkedList<T>).as_bytes()) ^ T::HASH;
    const TYPE: &'static str = "LinkedList<T>";
    const SIZE: Option<usize> = None;
}

impl<K: PayloadInfo, V: PayloadInfo> PayloadInfo for HashMap<K, V> {
    const HASH: u64 = PayloadConstHash("HashMap<K, V>".as_bytes()) ^ K::HASH ^ V::HASH;
    const TYPE: &'static str = "HashMap<K, V>";
}

impl<K: PayloadInfo, V: PayloadInfo> PayloadInfo for BTreeMap<K, V> {
    const HASH: u64 = PayloadConstHash("BTreeMap<K, V>".as_bytes()) ^ K::HASH ^ V::HASH;
    const TYPE: &'static str = "BTreeMap<K, V>";
}

impl<K: PayloadInfo> PayloadInfo for HashSet<K> {
    const HASH: u64 = PayloadConstHash("HashSet<K>".as_bytes()) ^ K::HASH;
    const TYPE: &'static str = "HashSet<K>";
}

impl<K: PayloadInfo> PayloadInfo for BTreeSet<K> {
    const HASH: u64 = PayloadConstHash("BTreeSet<K>".as_bytes()) ^ K::HASH;
    const TYPE: &'static str = "BTreeSet<K>";
}

impl<T: PayloadInfo + Ord> PayloadInfo for BinaryHeap<T> {
    const HASH: u64 = PayloadConstHash(stringify!(BinaryHeap<T>).as_bytes()) ^ T::HASH;
    const TYPE: &'static str = "BinaryHeap<T>";
    const SIZE: Option<usize> = None;
}

impl<T: PayloadInfo> PayloadInfo for Vec<T> {
    const HASH: u64 = PayloadConstHash(stringify!(&[T]).as_bytes()) ^ T::HASH;
    const TYPE: &'static str = "Vec<T>";
}

impl<'a, T: PayloadInfo> PayloadInfo for Cow<'a, [T]>  
    where T: Clone
{
    const HASH: u64 = PayloadConstHash(stringify!(&[T]).as_bytes()) ^ T::HASH;
    const TYPE: &'static str = "Cow";
}
