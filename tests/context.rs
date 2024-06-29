use npsd::{Error, FromPayload, IntoPayload, Middleware, Payload, PayloadContext, PayloadHandler, PayloadInfo};

#[derive(Clone, PartialEq, Debug)]
enum ContextUnwrap {
    Void,
    Usize(usize),
}

#[derive(Clone, PartialEq, Debug)]
struct CheckContext {
    sender: Option<ContextUnwrap>,
    receiver: Option<ContextUnwrap>,
} 

impl<C: PayloadContext<Context = usize>> IntoPayload<C> for ContextUnwrap {
    fn into_payload<M: Middleware>(&self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        match self {
            ContextUnwrap::Void => next.into_payload(&0u8, handler, ctx),
            ContextUnwrap::Usize(value) => {
                next.into_payload(&1u8, handler, ctx)?;
                next.into_payload(&(value + ctx.unwrap()), handler, ctx)
            }
        }
    }
}

impl<'a, C: PayloadContext<Context = usize>> FromPayload<'a, C> for ContextUnwrap {
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where
            'a: 'b
    {
        match next.from_payload::<C, u8>(handler, ctx)? {
            0 => {
                Ok(ContextUnwrap::Void)
            },
            1 => {
                Ok(ContextUnwrap::Usize(next.from_payload::<C, usize>(handler, ctx)? + ctx.unwrap()))
            },
            _ => {
                panic!("Unknown type!!!");
            }
        }
    }
}

impl<'a, C: PayloadContext<Context = usize>> Payload<'a, C> for ContextUnwrap {}

impl PayloadInfo for ContextUnwrap {
    const TYPE: &'static str = "ContextUnwrap";
}

impl<C: PayloadContext<Context = usize>> IntoPayload<C> for CheckContext {
    fn into_payload<M: Middleware>(&self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        let mut this = self.clone();

        this.sender = Some(ContextUnwrap::Usize(*ctx.unwrap()));

        next.into_payload(&this.sender, handler, ctx)?;
        next.into_payload(&this.receiver, handler, ctx)
    }
}

impl<'a, C: PayloadContext<Context = usize>> FromPayload<'a, C> for CheckContext {
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where
            'a: 'b
    {
        Ok(CheckContext {
            sender: next.from_payload(handler, ctx)?,
            receiver: Some(ContextUnwrap::Usize(*ctx.unwrap()))
        })
    }
}

impl<'a, C: PayloadContext<Context = usize>> Payload<'a, C> for CheckContext {}

impl PayloadInfo for CheckContext {
    const TYPE: &'static str = "ContextUnwrap";
}

#[test]
fn test_context_unwrap() -> Result<(), Error> {
    let mut sender_ctx = 1337usize;

    let serialized = CheckContext {
        sender: None,
        receiver: None,
    }.into_packet(&mut sender_ctx, 1470)?;

    let mut receiver_ctx = 42usize;

    let deserialized = CheckContext::from_packet(&mut receiver_ctx, &serialized)?;

    assert_eq!(deserialized, CheckContext {
        sender: Some(ContextUnwrap::Usize(sender_ctx * 2 + receiver_ctx)),
        receiver: Some(ContextUnwrap::Usize(receiver_ctx)),
    });

    dbg!(deserialized);

    Ok(())
}