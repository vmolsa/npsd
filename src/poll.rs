use core::task::{Poll, Context};
use super::{Middleware, PayloadHandler, FromPayload, IntoPayload, PayloadContext, PayloadInfo, Error};

pub trait AsyncMiddleware: Middleware {
    fn poll_into_payload<'a, S, C: PayloadContext, T: AsyncIntoPayload<S, C>>(
        &mut self,
        value: &T,
        state: &mut S,
        cx: &mut Context<'_>, 
        handler: &mut PayloadHandler<'_>,
        ctx: &mut C
    ) -> Poll<Result<(), Error>>;
            
    fn poll_from_payload<'a, 'b, S, C: PayloadContext, T: AsyncFromPayload<'a, S, C>> (
        &mut self, 
        state: &mut S,
        cx: &mut Context<'_>, 
        handler: &'b mut PayloadHandler<'a>,
        ctx: &mut C,
    ) -> Poll<Result<T, Error>>
        where
            'a: 'b;
}

pub trait AsyncState<C: PayloadContext> {
    type Output: PayloadInfo;
    type State;

    fn state(ctx: &mut C) -> Self::State;
}

pub trait AsyncIntoPayload<S, C: PayloadContext>: IntoPayload<C> {
    fn poll_into_payload<M: AsyncMiddleware>(&self, state: &mut S, cx: &mut Context<'_>, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Poll<Result<(), Error>>;
}

pub trait AsyncFromPayload<'a, S, C: PayloadContext>: FromPayload<'a, C> {
    fn poll_from_payload<'b, M: AsyncMiddleware>(state: &mut S, cx: &mut Context<'_>, handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Poll<Result<Self, Error>>
        where
            'a: 'b;
}
