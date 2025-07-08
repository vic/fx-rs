use dyn_clone::{DynClone, clone_trait_object};

use crate::Fx;

// Handler trait: represents a function Fx<'f, A, U> -> Fx<'f, B, V>
pub trait Handler<'f, A, B, U, V>: DynClone + 'f
where
    A: Clone + 'f,
    B: Clone + 'f,
    U: Clone + 'f,
    V: Clone + 'f,
{
    fn handle(&self, e: Fx<'f, A, U>) -> Fx<'f, B, V>;
}

clone_trait_object!(<'f, A, B, U, V> Handler<'f, A, B, U, V>
where
    A: Clone + 'f,
    B: Clone + 'f,
    U: Clone + 'f,
    V: Clone + 'f,
);

impl<'f, A, B, U, V, F> Handler<'f, A, B, U, V> for F
where
    F: FnOnce(Fx<'f, A, U>) -> Fx<'f, B, V> + Clone + 'f,
    A: Clone + 'f,
    B: Clone + 'f,
    U: Clone + 'f,
    V: Clone + 'f,
{
    fn handle(&self, e: Fx<'f, A, U>) -> Fx<'f, B, V> {
        (self.clone())(e)
    }
}

impl<'f, S: Clone, V: Clone> Fx<'f, S, V> {
    pub fn via<T, U>(self, h: impl Handler<'f, S, T, V, U> + 'f) -> Fx<'f, T, U>
    where
        T: Clone,
        U: Clone,
    {
        h.handle(self)
    }
}
