use super::handler::Handler;
use crate::Fx;
use dyn_clone::{DynClone, clone_trait_object};

impl<'f, I, S, O> Ability<'f, I, S, O>
where
    O: Clone,
    S: Clone + 'f,
    I: Clone + 'f,
{
    pub fn request(i: I) -> Fx<'f, (Self, S), O>
    where
        I: Clone,
    {
        Fx::ctx().flat_map(move |f: Self| f.apply(i.clone()))
    }

    pub fn handler<F, B, V>(f: F) -> Handler<'f, (Self, B), B, V, V>
    where
        F: Fn(I) -> Fx<'f, S, O> + Clone + 'f,
        B: Clone,
        V: Clone,
    {
        Handler::new(move |e: Fx<'f, (Self, B), V>| e.provide_left(Self::new(f.clone())))
    }

    pub(crate) fn new<F>(f: F) -> Self
    where
        F: Fn(I) -> Fx<'f, S, O> + Clone + 'f,
    {
        Self(Box::new(f))
    }

    fn apply(&self, i: I) -> Fx<'f, S, O> {
        self.0(i)
    }

    pub fn clone_boxed(&self) -> Box<dyn Fn(I) -> Fx<'f, S, O> + 'f> {
        self.0.clone()
    }

    pub fn adapt<C, H, F, T, U, V, M>(self, imap: H, cmap: C, fmap: F) -> Ability<'f, T, M, U>
    where
        T: Clone + 'f,
        U: Clone,
        V: Clone,
        M: Clone,
        H: Fn(T) -> I + Clone + 'f,
        C: Fn(M) -> S + Clone + 'f,
        F: Fn(O) -> Fx<'f, M, U> + Clone + 'f,
    {
        Ability::new(move |t: T| self.apply(imap(t)).adapt(cmap.clone(), fmap.clone()))
    }
}

#[derive(Clone)]
pub struct Ability<'f, I, S, O: Clone>(Box<dyn CapFn<'f, I, S, O> + 'f>);

clone_trait_object!(<'f, I, S, O: Clone> CapFn<'f, I, S, O>);

trait CapFn<'f, I, S, O>: DynClone + Fn(I) -> Fx<'f, S, O>
where
    O: Clone + 'f,
    S: 'f,
{
}

impl<'f, I, S, O, F> CapFn<'f, I, S, O> for F
where
    F: Fn(I) -> Fx<'f, S, O> + Clone,
    O: Clone + 'f,
    S: 'f,
{
}
