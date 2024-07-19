use std::{borrow::Cow, collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque}, mem};
use std::{collections::{HashMap, HashSet}, hash::Hash};

use crate::AnyBox;

use super::{Error, Middleware, Payload, IntoPayload, FromPayload};

impl<'a, C, T: IntoPayload<C>> IntoPayload<C> for VecDeque<T> {
    #[inline]
    fn into_payload<'m, M: Middleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&self.len(), ctx)?;

        for item in self {
            next.into_payload(item, ctx)?;
        }

        Ok(())
    }
}

impl<'a, C, T: FromPayload<'a, C>> FromPayload<'a, C> for VecDeque<T> {
    #[inline]
    fn from_payload<M: Middleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let mut deque = VecDeque::new();
        let count: usize = next.from_payload(ctx)?;

        for _ in 0..count {
            deque.push_back(next.from_payload::<C, T>(ctx)?);
        }

        Ok(deque)
    }
}

impl<'a, C, T: Payload<'a, C>> Payload<'a, C> for VecDeque<T> {}

impl<'a, C, T: IntoPayload<C>> IntoPayload<C> for LinkedList<T> {
    fn into_payload<'m, M: Middleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error>{
        next.into_payload(&self.len(), ctx)?;

        for item in self {
            next.into_payload(item, ctx)?;
        }

        Ok(())
    }
}

impl<'a, C, T: FromPayload<'a, C>> FromPayload<'a, C> for LinkedList<T> {
    fn from_payload<M: Middleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let mut list = LinkedList::new();
        let count: usize = next.from_payload(ctx)?;

        for _ in 0..count {
            list.push_back(next.from_payload::<C, T>(ctx)?);
        }

        Ok(list)
    }
}

impl<'a, C, T: Payload<'a, C>> Payload<'a, C> for LinkedList<T> {}

impl<'a, C, K: IntoPayload<C>, V: IntoPayload<C>> IntoPayload<C> for HashMap<K, V> {
    fn into_payload<'m, M: Middleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error>{
        next.into_payload(&self.len(), ctx)?;

        for (key, value) in self {
            next.into_payload(key, ctx)?;
            next.into_payload(value, ctx)?;
        }

        Ok(())
    }
}

impl<'a, C, K: FromPayload<'a, C>, V: FromPayload<'a, C>> FromPayload<'a, C> for HashMap<K, V> 
    where K: Hash + Eq 
{
    fn from_payload<M: Middleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let mut map = HashMap::new();
        let count: usize = next.from_payload(ctx)?;

        for _ in 0..count {
            let key: K = next.from_payload(ctx)?;
            let value: V = next.from_payload(ctx)?;

            map.insert(key, value);
        }

        Ok(map)
    }
}

impl<'a, C, K: Payload<'a, C>, V: Payload<'a, C>> Payload<'a, C> for HashMap<K, V> 
    where K: Hash + Eq {}

impl<'a, C, K: IntoPayload<C>, V: IntoPayload<C>> IntoPayload<C> for BTreeMap<K, V> {
    fn into_payload<'m, M: Middleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error>{
        next.into_payload(&self.len(), ctx)?;

        for (key, value) in self {
            next.into_payload(key, ctx)?;
            next.into_payload(value, ctx)?;
        }

        Ok(())
    }
}

impl<'a, C, K: FromPayload<'a, C> + Ord, V: FromPayload<'a, C>> FromPayload<'a, C> for BTreeMap<K, V> 
    where K: Hash + Eq 
{
    fn from_payload<M: Middleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let mut map = BTreeMap::new();
        let count: usize = next.from_payload(ctx)?;

        for _ in 0..count {
            let key: K = next.from_payload(ctx)?;
            let value: V = next.from_payload(ctx)?;

            map.insert(key, value);
        }

        Ok(map)
    }
}

impl<'a, C, K: Payload<'a, C>, V: Payload<'a, C>> Payload<'a, C> for BTreeMap<K, V> 
    where K: Hash + Eq + Ord {}

impl<'a, C, K: IntoPayload<C>> IntoPayload<C> for HashSet<K> {
    fn into_payload<'m, M: Middleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error>{
        next.into_payload(&self.len(), ctx)?;

        for key in self {
            next.into_payload(key, ctx)?;
        }

        Ok(())
    }
}

impl<'a, C, K: FromPayload<'a, C>> FromPayload<'a, C> for HashSet<K> 
    where K: Hash + Eq 
{    
    fn from_payload<M: Middleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let mut set = HashSet::new();
        let count: usize = next.from_payload(ctx)?;

        for _ in 0..count {
            let key: K = next.from_payload(ctx)?;

            set.insert(key);
        }

        Ok(set)
    }
}

impl<'a, C, K: Payload<'a, C>> Payload<'a, C> for HashSet<K> 
    where K: Hash + Eq {}

impl<'a, C, K: IntoPayload<C>> IntoPayload<C> for BTreeSet<K> {
    fn into_payload<'m, M: Middleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error>{
        next.into_payload(&self.len(), ctx)?;

        for key in self {
            next.into_payload(key, ctx)?;
        }

        Ok(())
    }
}

impl<'a, C, K: FromPayload<'a, C>> FromPayload<'a, C> for BTreeSet<K> 
    where K: Hash + Eq + Ord
{    
    fn from_payload<M: Middleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let mut set = BTreeSet::new();
        let count: usize = next.from_payload(ctx)?;

        for _ in 0..count {
            let key: K = next.from_payload(ctx)?;

            set.insert(key);
        }

        Ok(set)
    }
}

impl<'a, C, K: Payload<'a, C>> Payload<'a, C> for BTreeSet<K> 
    where K: Hash + Eq + Ord {}

impl<'a, C, T: IntoPayload<C> + Ord> IntoPayload<C> for BinaryHeap<T> {
    fn into_payload<'m, M: Middleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&self.len(), ctx)?;

        for item in self {
            next.into_payload(item, ctx)?;
        }

        Ok(())
    }
}

impl<'a, C, T: FromPayload<'a, C> + Ord> FromPayload<'a, C> for BinaryHeap<T> {
    fn from_payload<M: Middleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let mut heap = BinaryHeap::new();
        let count: usize = next.from_payload(ctx)?;

        for _ in 0..count {
            heap.push(next.from_payload::<C, T>(ctx)?);
        }

        Ok(heap)
    }
}

impl<'a, C, T: Payload<'a, C> + Ord> Payload<'a, C> for BinaryHeap<T> {}

impl<'a, C, T: IntoPayload<C>> IntoPayload<C> for Vec<T> {
    #[inline]
    fn into_payload<'m, M: Middleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error>{
        next.into_payload(&self.as_slice(), ctx)
    }
}

impl<'a, C, T: FromPayload<'a, C>> FromPayload<'a, C> for Vec<T> 
    where T: Clone + 'a 
{
    #[inline]
    fn from_payload<M: Middleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        Ok(Vec::from(next.from_payload::<C, Cow<'a, [T]>>(ctx)?.into_owned()))
    }
}

impl<'a, C, T: Payload<'a, C> + AnyBox<'a>> Payload<'a, C> for Vec<T> 
    where T: Clone {}

impl<'a, C, T: IntoPayload<C>> IntoPayload<C> for Cow<'a, [T]> 
    where T: Clone 
{
    fn into_payload<'m, M: Middleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error>{
        if mem::size_of::<T>() == 1 {
            match self {
                Cow::Borrowed(slice) => {
                    next.into_payload(&slice.len(), ctx)?;
                    next.write(slice)?;
                }
                Cow::Owned(vec) => {
                    next.into_payload(&vec.len(), ctx)?;
                    next.write(&vec)?;
                }
            }
        } else {
            match self {
                Cow::Borrowed(slice) => {
                    next.into_payload(&slice.len(), ctx)?;

                    for elem in *slice {
                        next.into_payload(elem, ctx)?;
                    }
                }
                Cow::Owned(vec) => {
                    next.into_payload(&vec.len(), ctx)?;

                    for elem in vec {
                        next.into_payload(elem, ctx)?;
                    }
                }
            }
        }

        Ok(())
    }
}

impl<'a, C, T: FromPayload<'a, C>> FromPayload<'a, C> for Cow<'a, [T]> 
    where T: Clone 
{
    fn from_payload<M: Middleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let len: usize = next.from_payload(ctx)?;

        if mem::size_of::<T>() == 1 {
            Ok(Cow::Borrowed(next.read(len)?))
        } else {
            let mut vec = Vec::with_capacity(len);

            for _ in 0..len {
                vec.push(next.from_payload::<C, T>(ctx)?);
            }

            Ok(Cow::Owned(vec))
        }
    }
}

impl<'a, C, T: Payload<'a, C>> Payload<'a, C> for Cow<'a, [T]> 
    where T: Clone {}