use std::pin::Pin;

use crate::PayloadContext;

impl PayloadContext for () {
    type Context = ();

    fn unwrap(&mut self) -> &Self::Context {
        &()
    }
}

impl PayloadContext for usize {
    type Context = usize;

    fn unwrap(&mut self) -> &Self::Context {
        self
    }
}

impl<T: Unpin> PayloadContext for Pin<&mut T> {
    type Context = T;

    fn unwrap(&mut self) -> &Self::Context {
        self.as_mut().get_mut()
    }
}
