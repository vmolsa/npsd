use std::{cell::RefCell, rc::Rc};
use crate::{Error, FromPayload, IntoPayload, Middleware, PayloadContext, PayloadHandler, PayloadInfo};

const MAX_NESTED_DEPTH: usize = 255;

#[derive(Clone, Debug)]
pub struct TraceInfo {
    pub nested: usize,
    pub ctx: &'static str,
    pub next: Option<Rc<RefCell<TraceInfo>>>,
}

impl Default for TraceInfo {
    fn default() -> Self {
        Self {
            nested: 0,
            ctx: "root",
            next: None,
        }
    }
}

/// A tracing implementation of the `Middleware` trait.
///
/// This implementation is used to trace the serialization and deserialization process. It tracks
/// the context in which an error occurs and provides a trace of nested contexts, which is useful
/// for debugging complex payload processing.
///
/// # Constants
/// - `MAX_NESTED_DEPTH`: The maximum allowed depth for nested serialization/deserialization contexts.
///
/// # Methods
/// - `fn into_payload<C: PayloadContext, T: IntoPayload<C> + PayloadInfo>(
///       &mut self,
///       value: &T,
///       handler: &mut PayloadHandler<'_>,
///       ctx: &mut C
///   ) -> Result<(), Error>`:
///     - Converts a value into a payload of bytes. This method tracks the context and nested depth
///       during the serialization process and provides detailed error tracing.
/// - `fn from_payload<'a, 'b, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo>(
///       &mut self,
///       handler: &'b mut PayloadHandler<'a>,
///       ctx: &mut C
///   ) -> Result<T, Error>`:
///     - Converts a payload of bytes back into a value. This method tracks the context and nested depth
///       during the deserialization process and provides detailed error tracing.
impl Middleware for TraceInfo {
    fn into_payload<C: PayloadContext, T: IntoPayload<C> + PayloadInfo>(&mut self, value: &T, handler: &mut PayloadHandler<'_>, ctx: &mut C) -> Result<(), Error> {
        if self.nested > MAX_NESTED_DEPTH {
            return Err(Error::NestedDepthLimit(T::TYPE.to_string()))
        }

        let mut scope = TraceInfo {
            nested: self.nested + 1,
            ctx: T::TYPE,
            next: Some(Rc::new(RefCell::new(self.clone()))),
        };

        match (*value).into_payload(handler, ctx, &mut scope) {
            Ok(value) => Ok(value),
            Err(error) => {
                Err(match error {
                    Error::Traced(_, _) => error,
                    _ => {
                        let mut nodes = vec![scope.ctx];
                        let mut current = scope.next.clone();

                        while let Some(node) = current {
                            let node_ref = node.borrow();

                            nodes.push(node_ref.ctx);
                            current = node_ref.next.clone();
                        }

                        nodes.reverse();
                        Error::Traced(error.to_string(), nodes.join(" -> "))
                    }
                })
            }
        }
    }

    fn from_payload<'a, 'b, C: PayloadContext, T: FromPayload<'a, C> + PayloadInfo>(&mut self, handler: &'b mut PayloadHandler<'a>, ctx: &mut C) -> Result<T, Error>
        where
            'a: 'b
    {
        if self.nested > MAX_NESTED_DEPTH {
            return Err(Error::NestedDepthLimit(T::TYPE.to_string()))
        }

        let mut scope = TraceInfo {
            nested: self.nested + 1,
            ctx: T::TYPE,
            next: Some(Rc::new(RefCell::new(self.clone()))),
        };
        
        match T::from_payload(handler, ctx, &mut scope) {
            Ok(value) => Ok(value),
            Err(error) => {
                Err(match error {
                    Error::Traced(_, _) => error,
                    _ => {
                        let mut nodes = vec![scope.ctx];
                        let mut current = scope.next.clone();

                        while let Some(node) = current {
                            let node_ref = node.borrow();

                            nodes.push(node_ref.ctx);
                            current = node_ref.next.clone();
                        }

                        nodes.reverse();
                        Error::Traced(error.to_string(), nodes.join(" <- "))
                    }
                })
            }
        }
    }
}
