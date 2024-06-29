use std::{borrow::Cow, collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque}, mem};
use std::{collections::{HashMap, HashSet}, hash::Hash};

use super::{Error, Middleware, PayloadContext, PayloadHandler, PayloadInfo, Payload, IntoPayload, FromPayload, PayloadConstHash};

impl<C: PayloadContext, T: IntoPayload<C> + PayloadInfo> IntoPayload<C> for VecDeque<T> {
    #[inline]
    fn into_payload<'b, M: Middleware>(&self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error>{
        for item in self {
            next.into_payload(item, handler, ctx)?;
        }
        Ok(())
    }
}

impl<'a, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo> FromPayload<'a, C> for VecDeque<T> {
    #[inline]
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        let mut deque = VecDeque::new();
        while let Ok(item) = next.from_payload::<C, T>(handler, ctx) {
            deque.push_back(item);
        }
        Ok(deque)
    }
}

impl<'a, C: PayloadContext, T: Payload<'a, C> + PayloadInfo> Payload<'a, C> for VecDeque<T> {}

impl<T: PayloadInfo> PayloadInfo for VecDeque<T> {
    const HASH: u64 = PayloadConstHash(stringify!(VecDeque<T>).as_bytes()) ^ T::HASH;
    const TYPE: &'static str = "VecDeque<T>";
    const SIZE: Option<usize> = None;
}

impl<C: PayloadContext, T: IntoPayload<C> + PayloadInfo> IntoPayload<C> for LinkedList<T> {
    fn into_payload<'b, M: Middleware>(&self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error>{
        for item in self {
            next.into_payload(item, handler, ctx)?;
        }

        Ok(())
    }
}

impl<'a, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo> FromPayload<'a, C> for LinkedList<T> {
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        let mut list = LinkedList::new();

        while let Ok(item) = next.from_payload::<C, T>(handler, ctx) {
            list.push_back(item);
        }

        Ok(list)
    }
}

impl<'a, C: PayloadContext, T: Payload<'a, C> + PayloadInfo> Payload<'a, C> for LinkedList<T> {}

impl<T: PayloadInfo> PayloadInfo for LinkedList<T> {
    const HASH: u64 = PayloadConstHash(stringify!(LinkedList<T>).as_bytes()) ^ T::HASH;
    const TYPE: &'static str = "LinkedList<T>";
    const SIZE: Option<usize> = None;
}

impl<C: PayloadContext, K: IntoPayload<C> + PayloadInfo, V: IntoPayload<C> + PayloadInfo> IntoPayload<C> for HashMap<K, V> {
    fn into_payload<'b, M: Middleware>(&self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error>{
        next.into_payload(&self.len(), handler, ctx)?;

        for (key, value) in self {
            next.into_payload(key, handler, ctx)?;
            next.into_payload(value, handler, ctx)?;
        }

        Ok(())
    }
}

impl<'a, C: PayloadContext, K: FromPayload<'a, C> + PayloadInfo, V: FromPayload<'a, C> + PayloadInfo> FromPayload<'a, C> for HashMap<K, V> 
    where K: Hash + Eq 
{
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        let mut map = HashMap::new();
        let count: usize = next.from_payload(handler, ctx)?;

        for _ in 0..count {
            let key: K = next.from_payload(handler, ctx)?;
            let value: V = next.from_payload(handler, ctx)?;

            map.insert(key, value);
        }

        Ok(map)
    }
}

impl<'a, C: PayloadContext, K: Payload<'a, C> + PayloadInfo, V: Payload<'a, C> + PayloadInfo> Payload<'a, C> for HashMap<K, V> 
    where K: Hash + Eq {}

impl<K: PayloadInfo, V: PayloadInfo> PayloadInfo for HashMap<K, V> {
    const HASH: u64 = PayloadConstHash("HashMap<K, V>".as_bytes()) ^ K::HASH ^ V::HASH;
    const TYPE: &'static str = "HashMap<K, V>";
}

impl<'a, C: PayloadContext, K: IntoPayload<C> + PayloadInfo, V: IntoPayload<C> + PayloadInfo> IntoPayload<C> for BTreeMap<K, V> {
    fn into_payload<'b, M: Middleware>(&self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error>{
        next.into_payload(&self.len(), handler, ctx)?;

        for (key, value) in self {
            next.into_payload(key, handler, ctx)?;
            next.into_payload(value, handler, ctx)?;
        }

        Ok(())
    }
}

impl<'a, C: PayloadContext, K: FromPayload<'a, C> + PayloadInfo + Ord, V: FromPayload<'a, C> + PayloadInfo> FromPayload<'a, C> for BTreeMap<K, V> 
    where K: Hash + Eq 
{
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        let mut map = BTreeMap::new();
        let count: usize = next.from_payload(handler, ctx)?;

        for _ in 0..count {
            let key: K = next.from_payload(handler, ctx)?;
            let value: V = next.from_payload(handler, ctx)?;

            map.insert(key, value);
        }

        Ok(map)
    }
}

impl<'a, C: PayloadContext, K: Payload<'a, C> + PayloadInfo, V: Payload<'a, C> + PayloadInfo> Payload<'a, C> for BTreeMap<K, V> 
    where K: Hash + Eq + Ord {}

impl<K: PayloadInfo, V: PayloadInfo> PayloadInfo for BTreeMap<K, V> {
    const HASH: u64 = PayloadConstHash("BTreeMap<K, V>".as_bytes()) ^ K::HASH ^ V::HASH;
    const TYPE: &'static str = "BTreeMap<K, V>";
}

impl<'a, C: PayloadContext, K: IntoPayload<C> + PayloadInfo> IntoPayload<C> for HashSet<K> {
    fn into_payload<'b, M: Middleware>(&self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error>{
        next.into_payload(&self.len(), handler, ctx)?;

        for key in self {
            next.into_payload(key, handler, ctx)?;
        }

        Ok(())
    }
}

impl<'a, C: PayloadContext, K: FromPayload<'a, C> + PayloadInfo> FromPayload<'a, C> for HashSet<K> 
    where K: Hash + Eq 
{    
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        let mut set = HashSet::new();
        let count: usize = next.from_payload(handler, ctx)?;

        for _ in 0..count {
            let key: K = next.from_payload(handler, ctx)?;

            set.insert(key);
        }

        Ok(set)
    }
}

impl<'a, C: PayloadContext, K: Payload<'a, C> + PayloadInfo> Payload<'a, C> for HashSet<K> 
    where K: Hash + Eq {}

impl<K: PayloadInfo> PayloadInfo for HashSet<K> {
    const HASH: u64 = PayloadConstHash("HashSet<K>".as_bytes()) ^ K::HASH;
    const TYPE: &'static str = "HashSet<K>";
}

impl<'a, C: PayloadContext, K: IntoPayload<C> + PayloadInfo> IntoPayload<C> for BTreeSet<K> {
    fn into_payload<'b, M: Middleware>(&self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error>{
        next.into_payload(&self.len(), handler, ctx)?;

        for key in self {
            next.into_payload(key, handler, ctx)?;
        }

        Ok(())
    }
}

impl<'a, C: PayloadContext, K: FromPayload<'a, C> + PayloadInfo> FromPayload<'a, C> for BTreeSet<K> 
    where K: Hash + Eq + Ord
{    
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        let mut set = BTreeSet::new();
        let count: usize = next.from_payload(handler, ctx)?;

        for _ in 0..count {
            let key: K = next.from_payload(handler, ctx)?;

            set.insert(key);
        }

        Ok(set)
    }
}

impl<'a, C: PayloadContext, K: Payload<'a, C> + PayloadInfo> Payload<'a, C> for BTreeSet<K> 
    where K: Hash + Eq + Ord {}

impl<K: PayloadInfo> PayloadInfo for BTreeSet<K> {
    const HASH: u64 = PayloadConstHash("BTreeSet<K>".as_bytes()) ^ K::HASH;
    const TYPE: &'static str = "BTreeSet<K>";
}

impl<'a, C: PayloadContext, T: IntoPayload<C> + PayloadInfo + Ord> IntoPayload<C> for BinaryHeap<T> {
    fn into_payload<'b, M: Middleware>(&self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error>{
        for item in self {
            next.into_payload(item, handler, ctx)?;
        }

        Ok(())
    }
}

impl<'a, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo + Ord> FromPayload<'a, C> for BinaryHeap<T> {
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        let mut heap = BinaryHeap::new();

        while let Ok(item) = next.from_payload::<C, T>(handler, ctx) {
            heap.push(item);
        }

        Ok(heap)
    }
}

impl<'a, C: PayloadContext, T: Payload<'a, C> + PayloadInfo + Ord> Payload<'a, C> for BinaryHeap<T> {}

impl<T: PayloadInfo + Ord> PayloadInfo for BinaryHeap<T> {
    const HASH: u64 = PayloadConstHash(stringify!(BinaryHeap<T>).as_bytes()) ^ T::HASH;
    const TYPE: &'static str = "BinaryHeap<T>";
    const SIZE: Option<usize> = None;
}

impl<'a, C: PayloadContext, T: IntoPayload<C> + PayloadInfo> IntoPayload<C> for Vec<T> {
    #[inline]
    fn into_payload<'b, M: Middleware>(&self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error>{
        next.into_payload(&self.as_slice(), handler, ctx)
    }
}

impl<'a, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo> FromPayload<'a, C> for Vec<T> 
    where T: Clone + 'a 
{
    #[inline]
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(Vec::from(next.from_payload::<C, Cow<'a, [T]>>(handler, ctx)?.into_owned()))
    }
}

impl<'a, C: PayloadContext, T: Payload<'a, C> + PayloadInfo> Payload<'a, C> for Vec<T> 
    where T: Clone + 'a {}

impl<T: PayloadInfo> PayloadInfo for Vec<T> {
    const HASH: u64 = PayloadConstHash(stringify!(&[T]).as_bytes()) ^ T::HASH;
    const TYPE: &'static str = "Vec<T>";
}

impl<'a, C: PayloadContext, T: IntoPayload<C> + PayloadInfo> IntoPayload<C> for Cow<'_, [T]> 
    where T: Clone 
{
    fn into_payload<'b, M: Middleware>(&self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error>{
        if mem::size_of::<T>() == 1 {
            match self {
                Cow::Borrowed(slice) => {
                    next.into_payload(&slice.len(), handler, ctx)?;
                    next.write(handler, slice)?;
                }
                Cow::Owned(vec) => {
                    next.into_payload(&vec.len(), handler, ctx)?;
                    next.write(handler, &vec)?;
                }
            }
        } else {
            match self {
                Cow::Borrowed(slice) => {
                    next.into_payload(&slice.len(), handler, ctx)?;

                    for elem in *slice {
                        next.into_payload(elem, handler, ctx)?;
                    }
                }
                Cow::Owned(vec) => {
                    next.into_payload(&vec.len(), handler, ctx)?;

                    for elem in vec {
                        next.into_payload(elem, handler, ctx)?;
                    }
                }
            }
        }

        Ok(())
    }
}

impl<'a, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo> FromPayload<'a, C> for Cow<'a, [T]> 
    where T: Clone 
{
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        let len: usize = next.from_payload(handler, ctx)?;

        if mem::size_of::<T>() == 1 {
            Ok(Cow::Borrowed(next.read(handler, len)?))
        } else {
            let mut vec = Vec::with_capacity(len);

            for _ in 0..len {
                vec.push(next.from_payload::<C, T>(handler, ctx)?);
            }

            Ok(Cow::Owned(vec))
        }
    }
}

impl<'a, C: PayloadContext, T: Payload<'a, C> + PayloadInfo> Payload<'a, C> for Cow<'a, [T]> 
    where T: Clone {}

impl<'a, T: PayloadInfo> PayloadInfo for Cow<'a, [T]>  
    where T: Clone
{
    const HASH: u64 = PayloadConstHash(stringify!(&[T]).as_bytes()) ^ T::HASH;
    const TYPE: &'static str = "Cow";
}
