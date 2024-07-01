use core::task::{Poll, Context};
use super::{Middleware, PayloadHandler, FromPayload, IntoPayload, PayloadContext, PayloadInfo, Error};

pub trait AsyncMiddleware: Middleware {
    fn poll_into_payload<'a, C: PayloadContext, S: AsyncIntoPayload<C>>(
        &mut self, 
        state: &mut S,
        cx: &mut Context<'_>, 
        handler: &mut PayloadHandler<'_>, 
    ) -> Poll<Result<(), Error>>;
            
    fn poll_from_payload<'a, 'b, S, C: PayloadContext, T: AsyncFromPayload<'a, S, C>> (
        &mut self, 
        state: &mut S,
        cx: &mut Context<'_>, 
        handler: &'b mut PayloadHandler<'a>, 
    ) -> Poll<Result<T, Error>>
        where
            'a: 'b;
}

pub trait AsyncIntoState<'a, C: PayloadContext>: PayloadInfo {
    type State: AsyncIntoPayload<C>;

    fn into_state(&'a self) -> Self::State;
}

pub trait AsyncFromState<'a> {
    type Output: PayloadInfo;
    type State;

    fn from_state() -> Self::State;
}

pub trait AsyncIntoPayload<C: PayloadContext>: IntoPayload<C> {
    fn poll_into_payload<M: AsyncMiddleware>(&mut self, cx: &mut Context<'_>, handler: &mut PayloadHandler<'_>, next: &mut M) -> Poll<Result<(), Error>>;
}

pub trait AsyncFromPayload<'a, State, C: PayloadContext>: FromPayload<'a, C> {
    fn poll_from_payload<'b, M: AsyncMiddleware>(state: &mut State, cx: &mut Context<'_>, handler: &'b mut PayloadHandler<'a>, next: &'b mut M) -> Poll<Result<Self, Error>>
        where
            'a: 'b;
}
