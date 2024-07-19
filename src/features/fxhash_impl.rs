use std::hash::Hash;
use fxhash::*;

#[cfg(feature = "sync")]
use crate::{Middleware, Payload, IntoPayload, FromPayload};

#[cfg(feature = "async")]
use crate::{AsyncMiddleware, AsyncPayload, AsyncIntoPayload, AsyncFromPayload};

use crate::{Error, PayloadInfo, PayloadConstHash};

#[cfg(feature = "sync")]
impl<'a, C, K: IntoPayload<C>, V: IntoPayload<C>> IntoPayload<C> for FxHashMap<K, V> {
    fn into_payload<'m, M: Middleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&self.len(), ctx)?;

        for (key, value) in self {
            next.into_payload(key, ctx)?;
            next.into_payload(value, ctx)?;
        }

        Ok(())
    }
} 

#[cfg(feature = "sync")]
impl<'a, C, K: FromPayload<'a, C>, V: FromPayload<'a, C>> FromPayload<'a, C> for FxHashMap<K, V> 
    where K: Hash + Eq 
{
    fn from_payload<M: Middleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let mut map = FxHashMap::default();
        let count: usize = next.from_payload(ctx)?;

        for _ in 0..count {
            let key: K = next.from_payload(ctx)?;
            let value: V = next.from_payload(ctx)?;

            map.insert(key, value);
        }

        Ok(map)
    }
}

#[cfg(feature = "sync")]
impl<'a, C, K: Payload<'a, C>, V: Payload<'a, C>> Payload<'a, C> for FxHashMap<K, V> 
    where K: Hash + Eq {}

#[cfg(feature = "async")]
impl<C: Send + Sync, K: AsyncIntoPayload<C>, V: AsyncIntoPayload<C>> AsyncIntoPayload<C> for FxHashMap<K, V> {
    async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.poll_into_payload(&self.len(), ctx).await?;

        for (key, value) in self {
            next.poll_into_payload(key, ctx).await?;
            next.poll_into_payload(value, ctx).await?;
        }

        Ok(())
    }
} 

#[cfg(feature = "async")]
impl<'a, C: Send + Sync, K: AsyncFromPayload<'a, C>, V: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for FxHashMap<K, V> 
    where K: Hash + Eq 
{
    async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let mut map = FxHashMap::default();
        let count: usize = next.poll_from_payload(ctx).await?;

        for _ in 0..count {
            let key: K = next.poll_from_payload(ctx).await?;
            let value: V = next.poll_from_payload(ctx).await?;

            map.insert(key, value);
        }

        Ok(map)
    }
}

#[cfg(feature = "async")]
impl<'a, C: Send + Sync, K: AsyncPayload<'a, C>, V: AsyncPayload<'a, C>> AsyncPayload<'a, C> for FxHashMap<K, V> 
    where K: Hash + Eq {}

impl<K: PayloadInfo, V: PayloadInfo> PayloadInfo for FxHashMap<K, V> {
    const HASH: u64 = PayloadConstHash("HashMap<K, V>".as_bytes()) ^ K::HASH ^ V::HASH;
    const TYPE: &'static str = "FxHashMap<K, V> ";
}

#[cfg(feature = "sync")]
impl<'a, C, K: IntoPayload<C>> IntoPayload<C> for FxHashSet<K> {
    fn into_payload<'m, M: Middleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&self.len(), ctx)?;

        for key in self {
            next.into_payload(key, ctx)?;
        }

        Ok(())
    }
}

#[cfg(feature = "sync")]
impl<'a, C, K: FromPayload<'a, C>> FromPayload<'a, C> for FxHashSet<K> 
    where K: Hash + Eq 
{
    fn from_payload<M: Middleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let mut set = FxHashSet::default();
        let count: usize = next.from_payload(ctx)?;

        for _ in 0..count {
            let key: K = next.from_payload(ctx)?;

            set.insert(key);
        }

        Ok(set)
    }
}

#[cfg(feature = "sync")]
impl<'a, C, K: Payload<'a, C>> Payload<'a, C> for FxHashSet<K> 
    where K: Hash + Eq {}

impl<K: PayloadInfo> PayloadInfo for FxHashSet<K> {
    const HASH: u64 = PayloadConstHash("HashSet<K>".as_bytes()) ^ K::HASH;
    const TYPE: &'static str = "FxHashSet<K>";
}

#[cfg(feature = "async")]
impl<C: Send + Sync, K: AsyncIntoPayload<C>> AsyncIntoPayload<C> for FxHashSet<K> {
    async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.poll_into_payload(&self.len(), ctx).await?;

        for key in self {
            next.poll_into_payload(key, ctx).await?;
        }

        Ok(())
    }
}

#[cfg(feature = "async")]
impl<'a, C: Send + Sync, K: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for FxHashSet<K> 
    where K: Hash + Eq 
{
    async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        let mut set = FxHashSet::default();
        let count: usize = next.poll_from_payload(ctx).await?;

        for _ in 0..count {
            let key: K = next.poll_from_payload(ctx).await?;

            set.insert(key);
        }

        Ok(set)
    }
}

#[cfg(feature = "async")]
impl<'a, C: Send + Sync, K: AsyncPayload<'a, C>> AsyncPayload<'a, C> for FxHashSet<K> 
    where K: Hash + Eq {}