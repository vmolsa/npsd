use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use super::{Error, Middleware, Payload, IntoPayload, FromPayload};

impl<'a, C> IntoPayload<C> for Ipv4Addr {
    #[inline]
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&self.octets(), ctx)
    }
}

impl<'a, C> FromPayload<'a, C> for Ipv4Addr {
    #[inline]
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(Ipv4Addr::from(next.from_payload::<C, [u8; 4]>(ctx)?))
    }
}

impl<C> Payload<C> for Ipv4Addr {}

impl<C> IntoPayload<C> for Ipv6Addr {
    #[inline]
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&self.octets(), ctx)
    }
}

impl<'a, C> FromPayload<'a, C> for Ipv6Addr {
    #[inline]
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(Ipv6Addr::from(next.from_payload::<C, [u8; 16]>(ctx)?))
    }
}

impl<C> Payload<C> for Ipv6Addr {}

impl<C> IntoPayload<C> for IpAddr {
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        match self {
            IpAddr::V4(ipv4) => {
                next.into_payload(&4u8, ctx)?;
                next.into_payload(ipv4, ctx)
            },
            IpAddr::V6(ipv6) => {
                next.into_payload(&6u8, ctx)?;
                next.into_payload(ipv6, ctx)
            },
        }
    }
}

impl<C> Payload<C> for IpAddr {}

impl<'a, C> FromPayload<'a, C> for IpAddr {
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        match next.from_payload::<C, u8>(ctx)? {
            4 => {
                Ok(IpAddr::V4(next.from_payload(ctx)?))
            },
            6 => {
                Ok(IpAddr::V6(next.from_payload(ctx)?))
            },
            _ => Err(Error::UnknownVariant("Invalid IP address format".to_string())),
        }
    }
}

impl<C> IntoPayload<C> for SocketAddr {
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&self.ip(), ctx)?;
        next.into_payload(&self.port(), ctx)
    }
}

impl<'a, C> FromPayload<'a, C> for SocketAddr {
    #[inline]
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(SocketAddr::new(next.from_payload(ctx)?, next.from_payload(ctx)?))
    }
}

impl<C> Payload<C> for SocketAddr {}
