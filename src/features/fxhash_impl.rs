use std::hash::Hash;
use fxhash::*;

use crate::{Error, Middleware, PayloadContext, PayloadHandler, PayloadInfo, Payload, IntoPayload, FromPayload, PayloadConstHash};

impl<C: PayloadContext, K: IntoPayload<C> + PayloadInfo, V: IntoPayload<C> + PayloadInfo> IntoPayload<C> for FxHashMap<K, V> {
    fn into_payload<M: Middleware>(&self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&self.len(), handler, ctx)?;

        for (key, value) in self {
            next.into_payload(key, handler, ctx)?;
            next.into_payload(value, handler, ctx)?;
        }

        Ok(())
    }
} 

impl<'a, C: PayloadContext, K: FromPayload<'a, C> + PayloadInfo, V: FromPayload<'a, C> + PayloadInfo> FromPayload<'a, C> for FxHashMap<K, V> 
    where K: Hash + Eq 
{
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
    where
        'a: 'b,
    {
        let mut map = FxHashMap::default();
        let count: usize = next.from_payload(handler, ctx)?;

        for _ in 0..count {
            let key: K = next.from_payload(handler, ctx)?;
            let value: V = next.from_payload(handler, ctx)?;

            map.insert(key, value);
        }

        Ok(map)
    }
}

impl<'a, C: PayloadContext, K: Payload<'a, C> + PayloadInfo, V: Payload<'a, C> + PayloadInfo> Payload<'a, C> for FxHashMap<K, V> 
    where K: Hash + Eq {}

impl<K: PayloadInfo, V: PayloadInfo> PayloadInfo for FxHashMap<K, V> {
    const HASH: u64 = PayloadConstHash("HashMap<K, V>".as_bytes()) ^ K::HASH ^ V::HASH;
    const TYPE: &'static str = "FxHashMap<K, V> ";
}

impl<C: PayloadContext, K: IntoPayload<C> + PayloadInfo> IntoPayload<C> for FxHashSet<K> {
    fn into_payload<M: Middleware>(&self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&self.len(), handler, ctx)?;

        for key in self {
            next.into_payload(key, handler, ctx)?;
        }

        Ok(())
    }
}

impl<'a, C: PayloadContext, K: FromPayload<'a, C> + PayloadInfo> FromPayload<'a, C> for FxHashSet<K> 
    where K: Hash + Eq 
{
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
    where
        'a: 'b,
    {
        let mut set = FxHashSet::default();
        let count: usize = next.from_payload(handler, ctx)?;

        for _ in 0..count {
            let key: K = next.from_payload(handler, ctx)?;

            set.insert(key);
        }

        Ok(set)
    }
}

impl<'a, C: PayloadContext, K: Payload<'a, C> + PayloadInfo> Payload<'a, C> for FxHashSet<K> 
    where K: Hash + Eq {}

impl<K: PayloadInfo> PayloadInfo for FxHashSet<K> {
    const HASH: u64 = PayloadConstHash("HashSet<K>".as_bytes()) ^ K::HASH;
    const TYPE: &'static str = "FxHashSet<K>";
}
