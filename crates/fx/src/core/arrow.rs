use super::handler::Handler;
use crate::{Fx, Pair, State};
use dyn_clone::{DynClone, clone_trait_object};

impl<'f, I, O> Arrow<'f, I, O>
where
    O: Clone,
    I: Clone + 'f,
{
    pub fn request(i: I) -> Fx<'f, Self, O>
    where
        I: Clone,
    {
        State::get().map(|f: Self| f.apply(i))
    }

    pub fn handler<B, V, P, F>(f: F) -> Handler<'f, P, B, V, V>
    where
        F: FnOnce(I) -> O + Clone + 'f,
        B: Clone,
        V: Clone,
        P: Pair<Self, B>,
    {
        Handler::new(|e| e.provide_left(Self::new(f)))
    }

    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(I) -> O + Clone + 'f,
    {
        Self(Box::new(f))
    }

    pub fn apply(self, i: I) -> O {
        self.0(i)
    }

    pub fn clone_boxed(&self) -> Box<dyn FnOnce(I) -> O + 'f> {
        self.0.clone()
    }

    pub fn adapt<T, U, H, F>(self, cmap: H, fmap: F) -> Arrow<'f, T, U>
    where
        T: Clone + 'f,
        U: Clone,
        H: FnOnce(T) -> I + Clone + 'f,
        F: FnOnce(O) -> U + Clone + 'f,
    {
        Arrow::new(|t: T| fmap(self.apply(cmap(t))))
    }
}

#[derive(Clone)]
pub struct Arrow<'f, I, O: Clone>(Box<dyn ArrowFn<'f, I, O> + 'f>);

clone_trait_object!(<'f, I, O: Clone> ArrowFn<'f, I, O>);

trait ArrowFn<'f, I, O>: DynClone + FnOnce(I) -> O
where
    O: Clone + 'f,
{
}

impl<'f, I, O, F> ArrowFn<'f, I, O> for F
where
    F: FnOnce(I) -> O + Clone,
    O: Clone + 'f,
{
}
