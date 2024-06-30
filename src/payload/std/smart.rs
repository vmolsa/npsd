use std::{borrow::Cow, cell::{Cell, Ref, RefCell, UnsafeCell}, pin::Pin, ptr, rc::Rc, sync::{Arc, Weak}};

use super::{Error, Middleware, PayloadContext, PayloadHandler, PayloadInfo, Payload, IntoPayload, FromPayload, PayloadConstHash};

macro_rules! impl_payload_smart_slice_traits {
    ($container:ident, $type_str:expr, $type_slice_str:expr) => {
        impl<'a, C: PayloadContext, T: IntoPayload<C> + PayloadInfo + 'a> IntoPayload<C> for $container<T> {
            #[inline]
            fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
                next.into_payload(self.as_ref(), handler, ctx)
            }
        }

        impl<'a, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo> FromPayload<'a, C> for $container<T> 
            where T: ToOwned 
        {
            #[inline]
            fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
                where 'a: 'b,
            {
                Ok($container::new(next.from_payload::<C, T>(handler, ctx)?))
            }
        }

        impl<'a, C: PayloadContext, T: Payload<'a, C> + PayloadInfo> Payload<'a, C> for $container<T> 
            where T: Clone + 'a {}

        impl<T: PayloadInfo> PayloadInfo for $container<T> {
            const HASH: u64 = T::HASH;
            const TYPE: &'static str = $type_str;
            const SIZE: Option<usize> = T::SIZE;
        }

        impl<'a, C: PayloadContext, T: IntoPayload<C> + PayloadInfo> IntoPayload<C> for $container<[T]> {
            #[inline]
            fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
                next.into_payload(&self.as_ref(), handler, ctx)
            }
        }

        impl<'a, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo> FromPayload<'a, C> for $container<[T]> 
            where T: Clone + 'a 
        {
            #[inline]
            fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
                where 'a: 'b,
            {
                Ok($container::from(next.from_payload::<C, Cow<'a, [T]>>(handler, ctx)?.into_owned()))
            }
        }

        impl<'a, C: PayloadContext, T: Payload<'a, C> + PayloadInfo> Payload<'a, C> for $container<[T]> 
            where T: Clone + 'a {}

        impl<T: PayloadInfo> PayloadInfo for $container<[T]> {
            const HASH: u64 = PayloadConstHash(stringify!(&[T]).as_bytes()) ^ T::HASH;
            const TYPE: &'static str = $type_slice_str;
        }
    };
}

impl_payload_smart_slice_traits!(Box, "Box<T>", "Box<[T]>");
impl_payload_smart_slice_traits!(Arc, "Arc<T>", "Arc<[T]>");
impl_payload_smart_slice_traits!(Rc, "Rc<T>", "Rc<[T]>");

impl<'a, C: PayloadContext, T: IntoPayload<C> + PayloadInfo + Copy> IntoPayload<C> for UnsafeCell<T> {
    #[inline]
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload::<C, T>(unsafe { &*self.get() }, handler, ctx)
    }
}

impl<'a, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo> FromPayload<'a, C> for UnsafeCell<T> {
    #[inline]
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(UnsafeCell::new(next.from_payload::<C, T>(handler, ctx)?))
    }
}

impl<'a, C: PayloadContext, T: Payload<'a, C> + PayloadInfo + Copy> Payload<'a, C> for UnsafeCell<T> {}

impl<T: PayloadInfo> PayloadInfo for UnsafeCell<T> {
    const HASH: u64 = T::HASH;
    const TYPE: &'static str = T::TYPE;
    const SIZE: Option<usize> = T::SIZE;
}

impl<'a, C: PayloadContext, T: IntoPayload<C> + PayloadInfo + Copy> IntoPayload<C> for Cell<T> {
    #[inline]
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload::<C, T>(&self.get(), handler, ctx)
    }
}

impl<'a, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo> FromPayload<'a, C> for Cell<T> {
    #[inline]
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(Cell::new(next.from_payload::<C, T>(handler, ctx)?))
    }
}

impl<'a, C: PayloadContext, T: Payload<'a, C> + PayloadInfo + Copy> Payload<'a, C> for Cell<T> {}

impl<T: PayloadInfo> PayloadInfo for Cell<T> {
    const HASH: u64 = T::HASH;
    const TYPE: &'static str = T::TYPE;
    const SIZE: Option<usize> = T::SIZE;
}

impl<'a, C: PayloadContext, T: IntoPayload<C> + PayloadInfo> IntoPayload<C> for Ref<'a, T> {
    #[inline]
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&**self, handler, ctx)
    }
}

impl<'a, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo> FromPayload<'a, C> for Ref<'a, T> {
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        let boxed_ref_cell = Box::new(RefCell::new(next.from_payload::<C, T>(handler, ctx)?));
        let cell: &'a RefCell<T> = Box::leak(boxed_ref_cell);

        let borrowed = cell.borrow();
        let borrowed_ptr: *const T = &*borrowed;

        Ok(unsafe { ptr::read(borrowed_ptr as *const Ref<'a, T>) })
    }
}

impl<'a, C: PayloadContext, T: Payload<'a, C> + PayloadInfo> Payload<'a, C> for Ref<'a, T> {}

impl<T: PayloadInfo> PayloadInfo for Ref<'_, T> {
    const HASH: u64 = T::HASH;
    const TYPE: &'static str = "Ref<T>";
    const SIZE: Option<usize> = T::SIZE;
}

impl<'a, C: PayloadContext, T: IntoPayload<C> + PayloadInfo> IntoPayload<C> for RefCell<T> {
    #[inline]
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(&*self.borrow(), handler, ctx)
    }
}

impl<'a, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo> FromPayload<'a, C> for RefCell<T> {
    #[inline]
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(RefCell::new(next.from_payload::<C, T>(handler, ctx)?))
    }
}

impl<'a, C: PayloadContext, T: Payload<'a, C> + PayloadInfo> Payload<'a, C> for RefCell<T> {}

impl<T: PayloadInfo> PayloadInfo for RefCell<T> {
    const HASH: u64 = T::HASH;
    const TYPE: &'static str = T::TYPE;
    const SIZE: Option<usize> = T::SIZE;
}

impl<'a, C: PayloadContext, T: IntoPayload<C> + PayloadInfo> IntoPayload<C> for Pin<Box<T>> {
    #[inline]
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        next.into_payload(self.as_ref().get_ref(), handler, ctx)
    }
}

impl<'a, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo> FromPayload<'a, C> for Pin<Box<T>> {
    #[inline]
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(Pin::from(Box::pin(next.from_payload::<C, T>(handler, ctx)?)))
    }
}

impl<'a, C: PayloadContext, T: Payload<'a, C> + PayloadInfo> Payload<'a, C> for Pin<Box<T>> {}

impl<T: PayloadInfo> PayloadInfo for Pin<Box<T>> {
    const HASH: u64 = T::HASH;
    const TYPE: &'static str = T::TYPE;
    const SIZE: Option<usize> = T::SIZE;
}

impl<'a, C: PayloadContext, T: IntoPayload<C> + PayloadInfo + 'a> IntoPayload<C> for Weak<T> {
    fn into_payload<'b, M: Middleware>(&'b self, handler: &mut PayloadHandler<'_>, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        if let Some(strong) = self.upgrade() {
            next.into_payload(&strong, handler, ctx)
        } else {
            Err(Error::WeakUpgrade)
        }
    }
}

impl<'a, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo + Clone> FromPayload<'a, C> for Weak<T> {
    #[inline]
    fn from_payload<'b, M: Middleware>(handler: &'b mut PayloadHandler<'a>, ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        Ok(Arc::downgrade(&next.from_payload::<C, Arc<T>>(handler, ctx)?))
    }
}

impl<'a, C: PayloadContext, T: Payload<'a, C> + PayloadInfo + Clone + 'a> Payload<'a, C> for Weak<T> {}

impl<T: PayloadInfo> PayloadInfo for Weak<T> {
    const HASH: u64 = T::HASH;
    const TYPE: &'static str = T::TYPE;
    const SIZE: Option<usize> = T::SIZE;
}