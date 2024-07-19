use std::{borrow::Cow, 
    // cell::{Cell, Ref, RefCell, UnsafeCell}, 
    pin::Pin, 
    // ptr, rc::Rc, 
    sync::{Arc, Weak}
};

use super::{Error, AsyncMiddleware, AsyncPayload, AsyncIntoPayload, AsyncFromPayload};

macro_rules! impl_payload_smart_slice_traits {
    ($container:ident) => {
        impl<C: Send + Sync, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for $container<T> {
            #[inline]
            async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
                next.poll_into_payload(self.as_ref(), ctx).await
            }
        }

        impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for $container<T> 
            where T: ToOwned 
        {
            #[inline]
            async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
                Ok($container::new(next.poll_from_payload::<C, T>(ctx).await?))
            }
        }

        impl<'a, C: Send + Sync, T: AsyncPayload<'a, C>> AsyncPayload<'a, C> for $container<T> 
            where T: Clone {}

        impl<C: Send + Sync, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for $container<[T]> {
            #[inline]
            async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
                next.poll_into_payload(&self.as_ref(), ctx).await
            }
        }

        impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for $container<[T]> 
            where T: Clone + 'a 
        {
            #[inline]
            async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
                Ok($container::from(next.poll_from_payload::<C, Cow<'a, [T]>>(ctx).await?.into_owned()))
            }
        }

        impl<'a, C: Send + Sync, T: AsyncPayload<'a, C>> AsyncPayload<'a, C> for $container<[T]> 
            where T: Clone {}
    };
}

impl_payload_smart_slice_traits!(Box);
impl_payload_smart_slice_traits!(Arc);

// // TODO(): Disabled because Send + Sync
// impl_payload_smart_slice_traits!(Rc);

// // TODO(): Disabled because Send + Sync
// impl<C: Send + Sync, T: AsyncIntoPayload<C> + Copy> AsyncIntoPayload<C> for UnsafeCell<T> {
//     #[inline]
//     async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
//         next.poll_into_payload::<C, T>(unsafe { &*self.get() }, ctx).await
//     }
// }

// impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for UnsafeCell<T> {
//     #[inline]
//     async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
//         Ok(UnsafeCell::new(next.poll_from_payload::<C, T>(ctx).await?))
//     }
// }

// impl<'a, C: Send + Sync, T: AsyncPayload<'a, C> + Copy> AsyncPayload<'a, C> for UnsafeCell<T> {}

// // TODO(): Disabled because Send + Sync
// impl<C: Send + Sync, T: AsyncIntoPayload<C> + Copy> AsyncIntoPayload<C> for Cell<T> {
//     #[inline]
//     async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
//         next.poll_into_payload::<C, T>(&self.get(), ctx).await
//     }
// }

// impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for Cell<T> {
//     #[inline]
//     async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
//         Ok(Cell::new(next.poll_from_payload::<C, T>(ctx).await?))
//     }
// }

// impl<'a, C: Send + Sync, T: AsyncPayload<'a, C> + Copy> AsyncPayload<'a, C> for Cell<T> {}

// impl<'a, C: Send + Sync, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for Ref<'a, T> {
//     #[inline]
//     async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
//         next.poll_into_payload(&**self, ctx).await
//     }
// }

// // TODO(): Disabled because Send + Sync
// impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for Ref<'a, T> {
//     async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
//         let boxed_ref_cell = Box::new(RefCell::new(next.poll_from_payload::<C, T>(ctx).await?));
//         let cell: &'a RefCell<T> = Box::leak(boxed_ref_cell);

//         let borrowed = cell.borrow();
//         let borrowed_ptr: *const T = &*borrowed;

//         Ok(unsafe { ptr::read(borrowed_ptr as *const Ref<'a, T>) })
//     }
// }

// // TODO(): Disabled because Send + Sync
// impl<'a, C: Send + Sync, T: AsyncPayload<'a, C>> AsyncPayload<'a, C> for Ref<'a, T> {}

// impl<C: Send + Sync, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for RefCell<T> {
//     #[inline]
//     async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
//         next.poll_into_payload(&*self.borrow(), ctx).await
//     }
// }

// impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for RefCell<T> {
//     #[inline]
//     async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
//         Ok(RefCell::new(next.poll_from_payload::<C, T>(ctx).await?))
//     }
// }

// impl<'a, C: Send + Sync, T: AsyncPayload<'a, C>> AsyncPayload<'a, C> for RefCell<T> {}

impl<C: Send + Sync, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for Pin<Box<T>> {
    #[inline]
    async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.poll_into_payload(self.as_ref().get_ref(), ctx).await
    }
}

impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for Pin<Box<T>> {
    #[inline]
    async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        Ok(Pin::from(Box::pin(next.poll_from_payload::<C, T>(ctx).await?)))
    }
}

impl<'a, C: Send + Sync, T: AsyncPayload<'a, C>> AsyncPayload<'a, C> for Pin<Box<T>> {}

impl<C: Send + Sync, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for Weak<T> {
    async fn poll_into_payload<'m, M: AsyncMiddleware<'m>>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        if let Some(strong) = self.upgrade() {
            next.poll_into_payload(&strong, ctx).await
        } else {
            Err(Error::WeakUpgrade)
        }
    }
}

impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C> + Clone> AsyncFromPayload<'a, C> for Weak<T> {
    #[inline]
    async fn poll_from_payload<M: AsyncMiddleware<'a>>(ctx: &mut C, next: &mut M) -> Result<Self, Error> {
        Ok(Arc::downgrade(&next.poll_from_payload::<C, Arc<T>>(ctx).await?))
    }
}

impl<'a, C: Send + Sync, T: AsyncPayload<'a, C> + Clone> AsyncPayload<'a, C> for Weak<T> {}
