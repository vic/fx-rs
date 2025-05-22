use crate::{And, Fx, Handler};
use dyn_clone::{DynClone, clone_trait_object};

impl<'f, I, O: Clone> Fx<'f, I, O> {
    pub fn apply<F>(i: I) -> Fx<'f, F, O>
    where
        I: Clone + 'f,
        F: Fn(I) -> O + Clone,
    {
        Fx::ctx().map(move |f: F| f(i.clone()))
    }
}

impl<'f, I, S, O> Ability<'f, I, S, O>
where
    O: Clone,
    S: Clone + 'f,
    I: Clone + 'f,
{
    pub fn request(i: I) -> Fx<'f, And<Self, S>, O>
    where
        I: Clone,
    {
        Fx::ctx().flat_map(move |f: Self| f.apply(i.clone()))
    }

    pub fn handler<F, B, V>(f: F) -> Handler<'f, And<Self, B>, B, V, V>
    where
        F: Fn(I) -> Fx<'f, S, O> + Clone + 'f,
        B: Clone,
        V: Clone,
    {
        Handler::new(move |e: Fx<'f, And<Self, B>, V>| e.provide_left(Self::new(f.clone())))
    }

    fn new<F>(f: F) -> Self
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
