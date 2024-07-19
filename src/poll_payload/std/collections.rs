use std::{borrow::Cow, collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque}, mem};
use std::{collections::{HashMap, HashSet}, hash::Hash};

use super::{Error, AsyncMiddleware, AsyncPayload, AsyncIntoPayload, AsyncFromPayload};

impl<C: Send + Sync, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for VecDeque<T> {
    #[inline]
    async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.poll_into_payload(&self.len(), ctx).await?;

        for item in self {
            next.poll_into_payload(item, ctx).await?;
        }

        Ok(())
    }
}

impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for VecDeque<T> {
    #[inline]
    async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let mut deque = VecDeque::new();
        let count: usize = next.poll_from_payload(ctx).await?;

        for _ in 0..count {
            deque.push_back(next.poll_from_payload::<C, T>(ctx).await?);
        }
        
        Ok(deque)
    }
}

impl<'a, C: Send + Sync, T: AsyncPayload<'a, C>> AsyncPayload<'a, C> for VecDeque<T> {}

impl<C: Send + Sync, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for LinkedList<T> {
    async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error>{
        next.poll_into_payload(&self.len(), ctx).await?;

        for item in self {
            next.poll_into_payload(item, ctx).await?;
        }

        Ok(())
    }
}

impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for LinkedList<T> {
    async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let mut list = LinkedList::new();
        let count: usize = next.poll_from_payload(ctx).await?;

        for _ in 0..count {
            list.push_back(next.poll_from_payload::<C, T>(ctx).await?);
        }

        Ok(list)
    }
}

impl<'a, C: Send + Sync, T: AsyncPayload<'a, C>> AsyncPayload<'a, C> for LinkedList<T> {}

impl<C: Send + Sync, K: AsyncIntoPayload<C>, V: AsyncIntoPayload<C>> AsyncIntoPayload<C> for HashMap<K, V> {
    async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error>{
        next.poll_into_payload(&self.len(), ctx).await?;

        for (key, value) in self {
            next.poll_into_payload(key, ctx).await?;
            next.poll_into_payload(value, ctx).await?;
        }

        Ok(())
    }
}

impl<'a, C: Send + Sync, K: AsyncFromPayload<'a, C>, V: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for HashMap<K, V> 
    where K: Hash + Eq 
{
    async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let mut map = HashMap::new();
        let count: usize = next.poll_from_payload(ctx).await?;

        for _ in 0..count {
            let key: K = next.poll_from_payload(ctx).await?;
            let value: V = next.poll_from_payload(ctx).await?;

            map.insert(key, value);
        }

        Ok(map)
    }
}

impl<'a, C: Send + Sync, K: AsyncPayload<'a, C>, V: AsyncPayload<'a, C>> AsyncPayload<'a, C> for HashMap<K, V> 
    where K: Hash + Eq {}

impl<C: Send + Sync, K: AsyncIntoPayload<C>, V: AsyncIntoPayload<C>> AsyncIntoPayload<C> for BTreeMap<K, V> {
    async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error>{
        next.poll_into_payload(&self.len(), ctx).await?;

        for (key, value) in self {
            next.poll_into_payload(key, ctx).await?;
            next.poll_into_payload(value, ctx).await?;
        }

        Ok(())
    }
}

impl<'a, C: Send + Sync, K: AsyncFromPayload<'a, C> + Ord, V: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for BTreeMap<K, V> 
    where K: Hash + Eq 
{
    async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let mut map = BTreeMap::new();
        let count: usize = next.poll_from_payload(ctx).await?;

        for _ in 0..count {
            let key: K = next.poll_from_payload(ctx).await?;
            let value: V = next.poll_from_payload(ctx).await?;

            map.insert(key, value);
        }

        Ok(map)
    }
}

impl<'a, C: Send + Sync, K: AsyncPayload<'a, C>, V: AsyncPayload<'a, C>> AsyncPayload<'a, C> for BTreeMap<K, V> 
    where K: Hash + Eq + Ord {}

impl<C: Send + Sync, K: AsyncIntoPayload<C>> AsyncIntoPayload<C> for HashSet<K> {
    async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error>{
        next.poll_into_payload(&self.len(), ctx).await?;

        for key in self {
            next.poll_into_payload(key, ctx).await?;
        }

        Ok(())
    }
}

impl<'a, C: Send + Sync, K: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for HashSet<K> 
    where K: Hash + Eq 
{    
    async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let mut set = HashSet::new();
        let count: usize = next.poll_from_payload(ctx).await?;

        for _ in 0..count {
            let key: K = next.poll_from_payload(ctx).await?;

            set.insert(key);
        }

        Ok(set)
    }
}

impl<'a, C: Send + Sync, K: AsyncPayload<'a, C>> AsyncPayload<'a, C> for HashSet<K> 
    where K: Hash + Eq {}

impl<C: Send + Sync, K: AsyncIntoPayload<C>> AsyncIntoPayload<C> for BTreeSet<K> {
    async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error>{
        next.poll_into_payload(&self.len(), ctx).await?;

        for key in self {
            next.poll_into_payload(key, ctx).await?;
        }

        Ok(())
    }
}

impl<'a, C: Send + Sync, K: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for BTreeSet<K> 
    where K: Hash + Eq + Ord
{    
    async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let mut set = BTreeSet::new();
        let count: usize = next.poll_from_payload(ctx).await?;

        for _ in 0..count {
            let key: K = next.poll_from_payload(ctx).await?;

            set.insert(key);
        }

        Ok(set)
    }
}

impl<'a, C: Send + Sync, K: AsyncPayload<'a, C>> AsyncPayload<'a, C> for BTreeSet<K> 
    where K: Hash + Eq + Ord {}

impl<C: Send + Sync, T: AsyncIntoPayload<C> + Ord> AsyncIntoPayload<C> for BinaryHeap<T> {
    async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.poll_into_payload(&self.len(), ctx).await?;

        for item in self {
            next.poll_into_payload(item, ctx).await?;
        }

        Ok(())
    }
}

impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C> + Ord> AsyncFromPayload<'a, C> for BinaryHeap<T> {
    async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let mut heap = BinaryHeap::new();
        let count: usize = next.poll_from_payload(ctx).await?;

        for _ in 0..count {
            heap.push(next.poll_from_payload::<C, T>(ctx).await?);
        }

        Ok(heap)
    }
}

impl<'a, C: Send + Sync, T: AsyncPayload<'a, C> + Ord> AsyncPayload<'a, C> for BinaryHeap<T> {}

impl<C: Send + Sync, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for Vec<T> {
    #[inline]
    async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error>{
        next.poll_into_payload(&self.as_slice(), ctx).await
    }
}

impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for Vec<T> 
    where T: Clone + 'a 
{
    #[inline]
    async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        Ok(Vec::from(next.poll_from_payload::<C, Cow<'a, [T]>>(ctx).await?.into_owned()))
    }
}

impl<'a, C: Send + Sync, T: AsyncPayload<'a, C>> AsyncPayload<'a, C> for Vec<T> 
    where T: Clone {}

impl<'a, C: Send + Sync, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for Cow<'a, [T]> 
    where T: Clone 
{
    async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        if mem::size_of::<T>() == 1 {
            match self {
                Cow::Borrowed(slice) => {
                    next.poll_into_payload(&slice.len(), ctx).await?;
                    next.poll_write(slice).await?;
                }
                Cow::Owned(vec) => {
                    next.poll_into_payload(&vec.len(), ctx).await?;
                    next.poll_write(&vec).await?;
                }
            }
        } else {
            match self {
                Cow::Borrowed(slice) => {
                    next.poll_into_payload(&slice.len(), ctx).await?;

                    for elem in *slice {
                        next.poll_into_payload(elem, ctx).await?;
                    }
                }
                Cow::Owned(vec) => {
                    next.poll_into_payload(&vec.len(), ctx).await?;

                    for elem in vec {
                        next.poll_into_payload(elem, ctx).await?;
                    }
                }
            }
        }

        Ok(())
    }
}

impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for Cow<'a, [T]> 
    where T: Clone 
{
    async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let len: usize = next.poll_from_payload(ctx).await?;

        if mem::size_of::<T>() == 1 {
            Ok(Cow::Borrowed(next.poll_read(len).await?))
        } else {
            let mut vec = Vec::with_capacity(len);

            for _ in 0..len {
                vec.push(next.poll_from_payload::<C, T>(ctx).await?);
            }

            Ok(Cow::Owned(vec))
        }
    }
}

impl<'a, C: Send + Sync, T: AsyncPayload<'a, C>> AsyncPayload<'a, C> for Cow<'a, [T]> 
    where T: Clone {}
