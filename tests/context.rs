#[cfg(feature = "sync")]
use npsd::{Payload, IntoPayload, FromPayload, Middleware, PayloadInfo, Error};

#[cfg(feature = "sync")]
#[derive(Clone, PartialEq, Debug)]
enum ContextUnwrap {
    Void,
    Usize(usize),
}

#[cfg(feature = "sync")]
#[derive(Clone, PartialEq, Debug)]
struct CheckContext {
    sender: Option<ContextUnwrap>,
    receiver: Option<ContextUnwrap>,
} 

#[cfg(feature = "sync")]
impl IntoPayload<usize> for ContextUnwrap {
    fn into_payload<'a, M: Middleware<'a>>(&self, ctx: &mut usize, next: &mut M) -> Result<(), Error> {
        match self {
            ContextUnwrap::Void => next.into_payload(&0u8, ctx),
            ContextUnwrap::Usize(value) => {
                next.into_payload(&1u8, ctx)?;
                next.into_payload(&(value + *ctx), ctx)
            }
        }
    }
}

#[cfg(feature = "sync")]
impl<'a> FromPayload<'a, usize> for ContextUnwrap {
    fn from_payload<M: Middleware<'a>>(ctx: &mut usize, next: &mut M) -> Result<Self, Error> {
        match next.from_payload::<usize, u8>(ctx)? {
            0 => {
                Ok(ContextUnwrap::Void)
            },
            1 => {
                Ok(ContextUnwrap::Usize(next.from_payload::<usize, usize>(ctx)? + *ctx))
            },
            _ => {
                panic!("Unknown type!!!");
            }
        }
    }
}

#[cfg(feature = "sync")]
impl<'a> Payload<'a, usize> for ContextUnwrap {}

#[cfg(feature = "sync")]
impl PayloadInfo for ContextUnwrap {
    const TYPE: &'static str = "ContextUnwrap";
}

#[cfg(feature = "sync")]
impl IntoPayload<usize> for CheckContext {
    fn into_payload<'a, M: Middleware<'a>>(&self, ctx: &mut usize, next: &mut M) -> Result<(), Error> {
        let mut this = self.clone();

        this.sender = Some(ContextUnwrap::Usize(*ctx));

        next.into_payload(&this.sender, ctx)?;
        next.into_payload(&this.receiver, ctx)
    }
}

#[cfg(feature = "sync")]
impl<'a> FromPayload<'a, usize> for CheckContext {
    fn from_payload<M: Middleware<'a>>(ctx: &mut usize, next: &mut M) -> Result<Self, Error> {
        Ok(CheckContext {
            sender: next.from_payload(ctx)?,
            receiver: Some(ContextUnwrap::Usize(*ctx))
        })
    }
}

#[cfg(feature = "sync")]
impl<'a> Payload<'a, usize> for CheckContext {}

#[cfg(feature = "sync")]
impl PayloadInfo for CheckContext {
    const TYPE: &'static str = "ContextUnwrap";
}

#[cfg(feature = "sync")]
#[test]
fn test_context_unwrap() -> Result<(), Error> {
    use pretty_hex::PrettyHex;

    #[cfg(feature = "info")]
    use npsd::NextTrace;

    #[cfg(not(feature = "info"))]
    use npsd::Next;

    // Create Middleware
    #[cfg(not(feature = "info"))]
    let mut next = Next::default();

    #[cfg(feature = "info")]
    let mut next = NextTrace::default();

    let mut sender_ctx = 1337usize;

    CheckContext {
        sender: None,
        receiver: None,
    }.into_packet(&mut sender_ctx, &mut next)?;

    println!("Encoded: {:?}", next.serialized().hex_dump());

    let mut receiver_ctx = 42usize;
    let deserialized = CheckContext::from_packet(&mut receiver_ctx, &mut next)?;    

    assert_eq!(deserialized, CheckContext {
        sender: Some(ContextUnwrap::Usize(sender_ctx * 2 + receiver_ctx)),
        receiver: Some(ContextUnwrap::Usize(receiver_ctx)),
    });

    dbg!(deserialized);

    Ok(())
}