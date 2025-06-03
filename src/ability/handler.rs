use crate::Fx;
use dyn_clone::{DynClone, clone_trait_object};

impl<'f, A, B, U: Clone, V: Clone> Handler<'f, A, B, U, V> {
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(Fx<'f, A, U>) -> Fx<'f, B, V> + Clone + 'f,
    {
        Self(Box::new(f))
    }

    pub fn handle(self, e: Fx<'f, A, U>) -> Fx<'f, B, V> {
        self.0(e)
    }

    pub fn clone_boxed(&self) -> Box<dyn FnOnce(Fx<'f, A, U>) -> Fx<'f, B, V> + 'f> {
        self.0.clone()
    }
}

#[derive(Clone)]
pub struct Handler<'f, A, B, U: Clone, V: Clone>(Box<dyn HandlerFn<'f, A, B, U, V> + 'f>);

clone_trait_object!(<'f, A, B, U:Clone, V:Clone> HandlerFn<'f, A, B, U, V>);
trait HandlerFn<'f, A, B, U, V>: DynClone + FnOnce(Fx<'f, A, U>) -> Fx<'f, B, V> + 'f
where
    V: Clone + 'f,
    U: Clone + 'f,
    A: 'f,
    B: 'f,
{
}
impl<'f, A, B, U: Clone, V: Clone, F> HandlerFn<'f, A, B, U, V> for F
where
    F: FnOnce(Fx<'f, A, U>) -> Fx<'f, B, V> + Clone + 'f,
    V: Clone + 'f,
    U: Clone + 'f,
    A: 'f,
    B: 'f,
{
}
