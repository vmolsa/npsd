
use std::sync::Arc;
use crossbeam::queue::SegQueue;

use crate::AnyBox;

#[derive(Clone, Debug)]
pub struct Stack<'a> {
    stack: Arc<SegQueue<Box<dyn AnyBox<'a> + 'a>>>,
}

impl<'a> Stack<'a> {
    pub fn new() -> Self {
        Self {
            stack: Arc::new(SegQueue::new()),
        }
    }
}

impl<'a> Stack<'a> {
    pub fn push<T: AnyBox<'a>>(&mut self, value: Box<T>) -> &'a T {
        let reference: &'a T = unsafe { &*(value.as_ref() as *const T) };
        self.stack.push(value);
        reference
    }

    pub fn push_mut<T: AnyBox<'a>>(&mut self, mut value: Box<T>) -> &'a mut T {
        let reference: &'a mut T = unsafe { &mut *(value.as_mut() as *mut T) };
        self.stack.push(value);
        reference
    }

    pub fn push_array<T: AnyBox<'a>>(&mut self, values: Box<[T]>) -> &'a [T] {
        let reference: &'a [T] = unsafe { &*(values.as_ref() as *const [T]) };
        self.stack.push(Box::new(values));
        reference
    }

    pub fn push_array_mut<T: AnyBox<'a>>(&mut self, mut values: Box<[T]>) -> &'a mut [T] {
        let reference: &'a mut [T] = unsafe { &mut *(values.as_mut() as *mut [T]) };
        self.stack.push(Box::new(values));
        reference
    }
}