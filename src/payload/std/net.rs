use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use super::{Error, Middleware, PayloadContext, PayloadHandler, PayloadInfo, Payload, IntoPayload, FromPayload};

impl<'a, C: PayloadContext> IntoPayload<C> for Ipv4Addr {
    #[inline]
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&self.octets(), handler, ctx)
    }
}

impl<'a, C: PayloadContext> FromPayload<'a, C> for Ipv4Addr {
    #[inline]
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(Ipv4Addr::from(next.from_payload::<C, [u8; 4]>(handler, ctx)?))
    }
}

impl<'a, C: PayloadContext> Payload<'a, C> for Ipv4Addr {}

impl PayloadInfo for Ipv4Addr {
    const TYPE: &'static str = "Ipv4Addr";
    const SIZE: Option<usize> = Some(std::mem::size_of::<Ipv4Addr>());
}

impl<'a, C: PayloadContext> IntoPayload<C> for Ipv6Addr {
    #[inline]
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&self.octets(), handler, ctx)
    }
}

impl<'a, C: PayloadContext> FromPayload<'a, C> for Ipv6Addr {
    #[inline]
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(Ipv6Addr::from(next.from_payload::<C, [u8; 16]>(handler, ctx)?))
    }
}

impl<'a, C: PayloadContext> Payload<'a, C> for Ipv6Addr {}

impl PayloadInfo for Ipv6Addr {
    const TYPE: &'static str = "Ipv6Addr";
    const SIZE: Option<usize> = Some(std::mem::size_of::<Ipv6Addr>());
}

impl<'a, C: PayloadContext> IntoPayload<C> for IpAddr {
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        match self {
            IpAddr::V4(ipv4) => {
                next.into_payload(&4u8, handler, ctx)?;
                next.into_payload(ipv4, handler, ctx)
            },
            IpAddr::V6(ipv6) => {
                next.into_payload(&6u8, handler, ctx)?;
                next.into_payload(ipv6, handler, ctx)
            },
        }
    }
}

impl<'a, C: PayloadContext> Payload<'a, C> for IpAddr {}

impl PayloadInfo for IpAddr {
    const TYPE: &'static str = "IpAddr";
    const SIZE: Option<usize> = Some(std::mem::size_of::<IpAddr>());
}

impl<'a, C: PayloadContext> FromPayload<'a, C> for IpAddr {
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        match next.from_payload::<C, u8>(handler, ctx)? {
            4 => {
                Ok(IpAddr::V4(next.from_payload(handler, ctx)?))
            },
            6 => {
                Ok(IpAddr::V6(next.from_payload(handler, ctx)?))
            },
            _ => Err(Error::UnknownVariant("Invalid IP address format".to_string())),
        }
    }
}

impl<'a, C: PayloadContext> IntoPayload<C> for SocketAddr {
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&self.ip(), handler, ctx)?;
        next.into_payload(&self.port(), handler, ctx)
    }
}

impl<'a, C: PayloadContext> FromPayload<'a, C> for SocketAddr {
    #[inline]
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(SocketAddr::new(next.from_payload(handler, ctx)?, next.from_payload(handler, ctx)?))
    }
}

impl<'a, C: PayloadContext> Payload<'a, C> for SocketAddr {}

impl PayloadInfo for SocketAddr {
    const TYPE: &'static str = "SocketAddr";
    const SIZE: Option<usize> = <[u8; 20]>::SIZE;
}