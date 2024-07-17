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
            async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
                next.poll_into_payload(self.as_ref(), ctx).await
            }
        }

        impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for $container<T> 
            where T: ToOwned 
        {
            #[inline]
            async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
                where 'a: 'b,
            {
                Ok($container::new(next.poll_from_payload::<C, T>(ctx).await?))
            }
        }

        impl<C: Send + Sync, T: AsyncPayload<C>> AsyncPayload<C> for $container<T> 
            where T: Clone {}

        impl<C: Send + Sync, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for $container<[T]> {
            #[inline]
            async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
                next.poll_into_payload(&self.as_ref(), ctx).await
            }
        }

        impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for $container<[T]> 
            where T: Clone + 'a 
        {
            #[inline]
            async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
                where 'a: 'b,
            {
                Ok($container::from(next.poll_from_payload::<C, Cow<'a, [T]>>(ctx).await?.into_owned()))
            }
        }

        impl<C: Send + Sync, T: AsyncPayload<C>> AsyncPayload<C> for $container<[T]> 
            where T: Clone {}
    };
}

impl_payload_smart_slice_traits!(Box);
impl_payload_smart_slice_traits!(Arc);

// TODO(): Disabled because Send + Sync
// impl_payload_smart_slice_traits!(Rc);

// TODO(): Disabled because Send + Sync
// impl<C: Send + Sync, T: AsyncIntoPayload<C> + Copy> AsyncIntoPayload<C> for UnsafeCell<T> {
//     #[inline]
//     async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
//         next.poll_into_payload::<C, T>(unsafe { &*self.get() }, ctx).await
//     }
// }

// impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for UnsafeCell<T> {
//     #[inline]
//     async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
//         where 'a: 'b,
//     {
//         Ok(UnsafeCell::new(next.poll_from_payload::<C, T>(ctx).await?))
//     }
// }

// impl<C: Send + Sync, T: AsyncPayload<C> + Copy> AsyncPayload<C> for UnsafeCell<T> {}

// TODO(): Disabled because Send + Sync
// impl<C: Send + Sync, T: AsyncIntoPayload<C> + Copy> AsyncIntoPayload<C> for Cell<T> {
//     #[inline]
//     async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
//         next.poll_into_payload::<C, T>(&self.get(), ctx).await
//     }
// }

// impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for Cell<T> {
//     #[inline]
//     async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
//         where 'a: 'b,
//     {
//         Ok(Cell::new(next.poll_from_payload::<C, T>(ctx).await?))
//     }
// }

// impl<C: Send + Sync, T: AsyncPayload<C> + Copy> AsyncPayload<C> for Cell<T> {}

// impl<'a, C: Send + Sync, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for Ref<'a, T> {
//     #[inline]
//     async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
//         next.poll_into_payload(&**self, ctx).await
//     }
// }

// TODO(): Disabled because Send + Sync
// impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for Ref<'a, T> {
//     async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
//         where 'a: 'b,
//     {
//         let boxed_ref_cell = Box::new(RefCell::new(next.poll_from_payload::<C, T>(ctx).await?));
//         // TODO(): Replace Box::leak()
//         let cell: &'a RefCell<T> = Box::leak(boxed_ref_cell);

//         let borrowed = cell.borrow();
//         let borrowed_ptr: *const T = &*borrowed;

//         Ok(unsafe { ptr::read(borrowed_ptr as *const Ref<'a, T>) })
//     }
// }

// TODO(): Disabled because Send + Sync
// impl<'a, C: Send + Sync, T: AsyncPayload<C>> AsyncPayload<C> for Ref<'a, T> {}

// impl<C: Send + Sync, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for RefCell<T> {
//     #[inline]
//     async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
//         next.poll_into_payload(&*self.borrow(), ctx).await
//     }
// }

// impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for RefCell<T> {
//     #[inline]
//     async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
//         where 'a: 'b,
//     {
//         Ok(RefCell::new(next.poll_from_payload::<C, T>(ctx).await?))
//     }
// }

// impl<C: Send + Sync, T: AsyncPayload<C>> AsyncPayload<C> for RefCell<T> {}

impl<C: Send + Sync, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for Pin<Box<T>> {
    #[inline]
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
        next.poll_into_payload(self.as_ref().get_ref(), ctx).await
    }
}

impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C>> AsyncFromPayload<'a, C> for Pin<Box<T>> {
    #[inline]
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(Pin::from(Box::pin(next.poll_from_payload::<C, T>(ctx).await?)))
    }
}

impl<C: Send + Sync, T: AsyncPayload<C>> AsyncPayload<C> for Pin<Box<T>> {}

impl<C: Send + Sync, T: AsyncIntoPayload<C>> AsyncIntoPayload<C> for Weak<T> {
    async fn poll_into_payload<'b, M: AsyncMiddleware>(&self, ctx: &mut C, next: &'b mut M) -> Result<(), Error> {
        if let Some(strong) = self.upgrade() {
            next.poll_into_payload(&strong, ctx).await
        } else {
            Err(Error::WeakUpgrade)
        }
    }
}

impl<'a, C: Send + Sync, T: AsyncFromPayload<'a, C> + Clone> AsyncFromPayload<'a, C> for Weak<T> {
    #[inline]
    async fn poll_from_payload<'b, M: AsyncMiddleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(Arc::downgrade(&next.poll_from_payload::<C, Arc<T>>(ctx).await?))
    }
}

impl<C: Send + Sync, T: AsyncPayload<C> + Clone> AsyncPayload<C> for Weak<T> {}
