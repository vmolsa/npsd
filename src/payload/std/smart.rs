use std::{borrow::Cow, cell::{Cell, Ref, RefCell, UnsafeCell}, pin::Pin, ptr, rc::Rc, sync::{Arc, Weak}};

use super::{Error, Middleware, Payload, IntoPayload, FromPayload};

macro_rules! impl_payload_smart_slice_traits {
    ($container:ident) => {
        impl<C, T: IntoPayload<C>> IntoPayload<C> for $container<T> {
            #[inline]
            fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
                next.into_payload(self.as_ref(), ctx)
            }
        }

        impl<'a, C, T: FromPayload<'a, C>> FromPayload<'a, C> for $container<T> 
            where T: ToOwned 
        {
            #[inline]
            fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
                where 'a: 'b,
            {
                Ok($container::new(next.from_payload::<C, T>(ctx)?))
            }
        }

        impl<C, T: Payload<C>> Payload<C> for $container<T> 
            where T: Clone {}

        impl<C, T: IntoPayload<C>> IntoPayload<C> for $container<[T]> {
            #[inline]
            fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
                next.into_payload(&self.as_ref(), ctx)
            }
        }

        impl<'a, C, T: FromPayload<'a, C>> FromPayload<'a, C> for $container<[T]> 
            where T: Clone + 'a 
        {
            #[inline]
            fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
                where 'a: 'b,
            {
                Ok($container::from(next.from_payload::<C, Cow<'a, [T]>>(ctx)?.into_owned()))
            }
        }

        impl<C, T: Payload<C>> Payload<C> for $container<[T]> 
            where T: Clone {}

    };
}

impl_payload_smart_slice_traits!(Box);
impl_payload_smart_slice_traits!(Arc);
impl_payload_smart_slice_traits!(Rc);

impl<C, T: IntoPayload<C> + Copy> IntoPayload<C> for UnsafeCell<T> {
    #[inline]
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload::<C, T>(unsafe { &*self.get() }, ctx)
    }
}

impl<'a, C, T: FromPayload<'a, C>> FromPayload<'a, C> for UnsafeCell<T> {
    #[inline]
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(UnsafeCell::new(next.from_payload::<C, T>(ctx)?))
    }
}

impl<C, T: Payload<C> + Copy> Payload<C> for UnsafeCell<T> {}

impl<C, T: IntoPayload<C> + Copy> IntoPayload<C> for Cell<T> {
    #[inline]
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload::<C, T>(&self.get(), ctx)
    }
}

impl<'a, C, T: FromPayload<'a, C>> FromPayload<'a, C> for Cell<T> {
    #[inline]
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(Cell::new(next.from_payload::<C, T>(ctx)?))
    }
}

impl<C, T: Payload<C> + Copy> Payload<C> for Cell<T> {}

impl<'a, C, T: IntoPayload<C>> IntoPayload<C> for Ref<'a, T> {
    #[inline]
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&**self, ctx)
    }
}

impl<'a, C, T: FromPayload<'a, C>> FromPayload<'a, C> for Ref<'a, T> {
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        let boxed_ref_cell = Box::new(RefCell::new(next.from_payload::<C, T>(ctx)?));
        let cell: &'a RefCell<T> = Box::leak(boxed_ref_cell);

        let borrowed = cell.borrow();
        let borrowed_ptr: *const T = &*borrowed;

        Ok(unsafe { ptr::read(borrowed_ptr as *const Ref<'a, T>) })
    }
}

impl<'a, C, T: Payload<C>> Payload<C> for Ref<'a, T> {}

impl<C, T: IntoPayload<C>> IntoPayload<C> for RefCell<T> {
    #[inline]
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&*self.borrow(), ctx)
    }
}

impl<'a, C, T: FromPayload<'a, C>> FromPayload<'a, C> for RefCell<T> {
    #[inline]
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(RefCell::new(next.from_payload::<C, T>(ctx)?))
    }
}

impl<C, T: Payload<C>> Payload<C> for RefCell<T> {}

impl<C, T: IntoPayload<C>> IntoPayload<C> for Pin<Box<T>> {
    #[inline]
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(self.as_ref().get_ref(), ctx)
    }
}

impl<'a, C, T: FromPayload<'a, C>> FromPayload<'a, C> for Pin<Box<T>> {
    #[inline]
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(Pin::from(Box::pin(next.from_payload::<C, T>(ctx)?)))
    }
}

impl<C, T: Payload<C>> Payload<C> for Pin<Box<T>> {}

impl<C, T: IntoPayload<C>> IntoPayload<C> for Weak<T> {
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        if let Some(strong) = self.upgrade() {
            next.into_payload(&strong, ctx)
        } else {
            Err(Error::WeakUpgrade)
        }
    }
}

impl<'a, C, T: FromPayload<'a, C> + Clone> FromPayload<'a, C> for Weak<T> {
    #[inline]
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(Arc::downgrade(&next.from_payload::<C, Arc<T>>(ctx)?))
    }
}

impl<C, T: Payload<C> + Clone> Payload<C> for Weak<T> {}
