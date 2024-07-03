use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use super::{Error, AsyncMiddleware, AsyncPayload, AsyncIntoPayload, AsyncFromPayload};

impl<C> AsyncIntoPayload<C> for Ipv4Addr {
    #[inline]
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
        next.poll_into_payload(&self.octets(), ctx).await
    }
}

impl<'a, C> AsyncFromPayload<'a, C> for Ipv4Addr {
    #[inline]
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(Ipv4Addr::from(next.poll_from_payload::<C, [u8; 4]>(ctx).await?))
    }
}

impl<C> AsyncPayload<C> for Ipv4Addr {}

impl<C> AsyncIntoPayload<C> for Ipv6Addr {
    #[inline]
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
        next.poll_into_payload(&self.octets(), ctx).await
    }
}

impl<'a, C> AsyncFromPayload<'a, C> for Ipv6Addr {
    #[inline]
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(Ipv6Addr::from(next.poll_from_payload::<C, [u8; 16]>(ctx).await?))
    }
}

impl<C> AsyncPayload<C> for Ipv6Addr {}

impl<C> AsyncIntoPayload<C> for IpAddr {
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
        match self {
            IpAddr::V4(ipv4) => {
                next.poll_into_payload(&4u8, ctx).await?;
                next.poll_into_payload(ipv4, ctx).await
            },
            IpAddr::V6(ipv6) => {
                next.poll_into_payload(&6u8, ctx).await?;
                next.poll_into_payload(ipv6, ctx).await
            },
        }
    }
}

impl<C> AsyncPayload<C> for IpAddr {}

impl<'a, C> AsyncFromPayload<'a, C> for IpAddr {
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        match next.poll_from_payload::<C, u8>(ctx).await? {
            4 => {
                Ok(IpAddr::V4(next.poll_from_payload(ctx).await?))
            },
            6 => {
                Ok(IpAddr::V6(next.poll_from_payload(ctx).await?))
            },
            _ => Err(Error::UnknownVariant("Invalid IP address format".to_string())),
        }
    }
}

impl<C> AsyncIntoPayload<C> for SocketAddr {
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
        next.poll_into_payload(&self.ip(), ctx).await?;
        next.poll_into_payload(&self.port(), ctx).await
    }
}

impl<'a, C> AsyncFromPayload<'a, C> for SocketAddr {
    #[inline]
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(SocketAddr::new(next.poll_from_payload(ctx).await?, next.poll_from_payload(ctx).await?))
    }
}

impl<C> AsyncPayload<C> for SocketAddr {}
