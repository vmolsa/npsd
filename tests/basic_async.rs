use npsd::{AsyncFromPayload, AsyncIntoPayload, AsyncMiddleware, AsyncState, Error, FromPayload, IntoPayload, PayloadContext, PayloadHandler, Schema};
use core::task::Context;
use core::future::poll_fn;
use std::task::Poll;

#[derive(Schema, PartialEq, Debug)]
enum Animal {
    Dog,
    Frog(String, Vec<isize>),
    Cat { age: usize, name: String },
    AntHive(Vec<String>),
}

#[derive(Default)]
struct AsyncAnimalState {
    
}

impl<C: PayloadContext> AsyncState<C> for Animal {
    type Output = Animal;
    type State = AsyncAnimalState;

    fn state(_: &mut C) -> Self::State {
        AsyncAnimalState {}
    }
}

impl<C: PayloadContext> AsyncIntoPayload<AsyncAnimalState, C> for Animal {
    fn poll_into_payload<M: AsyncMiddleware>(&self, state: &mut AsyncAnimalState, cx: &mut Context<'_>, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Poll<Result<(), Error>> {
        Poll::Ready(self.into_payload(handler, ctx, next))
    }
}

impl<'a, C: PayloadContext> AsyncFromPayload<'a, AsyncAnimalState, C> for Animal {
    fn poll_from_payload<'b, M: AsyncMiddleware>(state: &mut AsyncAnimalState, cx: &mut Context<'_>, handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Poll<Result<Self, Error>>
        where
            'a: 'b
    {
        Poll::Ready(Animal::from_payload(handler, ctx, next))
    }
}

#[tokio::test]
async fn test_async_serialization() {
    let animal = Animal::Frog("Frog".to_string(), vec![12393818, -19383812, 11111, -1093838482]);

    let mut middleware = ();
    let mut handler = PayloadHandler::from(vec![]);
    let mut state = Animal::state(&mut ());

    poll_fn(|cx| middleware.poll_into_payload(&animal, &mut state, cx, &mut handler, &mut ())).await.unwrap();

    let deserialized: Animal = poll_fn(|cx| middleware.poll_from_payload(&mut state, cx, &mut handler, &mut ())).await.unwrap();

    assert_eq!(deserialized, animal);
}