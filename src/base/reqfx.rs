use crate::{And, Fx, Handler};
use dyn_clone::{DynClone, clone_trait_object};

impl<'f, I, S, O> ReqFx<'f, I, S, O>
where
    O: Clone,
    S: 'f,
{
    pub fn suspend(i: I) -> Fx<'f, And<Self, S>, O>
    where
        I: Copy,
        S: Clone,
    {
        Fx::ctx().flat_map(move |f: Self| f.apply(i))
    }

    pub fn request(i: I) -> Fx<'f, And<Handler<'f, And<Self, S>, S, O, O>, S>, O>
    where
        I: Copy,
        S: Clone,
    {
        Fx::ctx().flat_map(move |h: Handler<'f, And<Self, S>, S, O, O>| h.handle(ReqFx::suspend(i)))
    }

    pub fn handler<F>(f: F) -> Handler<'f, And<Self, S>, S, O, O>
    where
        F: Fn(I) -> Fx<'f, S, O> + Copy + 'f,
        I: Clone,
        S: Clone,
    {
        Handler::new(move |e| e.provide_left(Self::new(f)))
    }

    pub fn new<F>(f: F) -> Self
    where
        F: Fn(I) -> Fx<'f, S, O> + Clone + 'f,
    {
        Self(Box::new(f))
    }

    pub fn apply(&self, i: I) -> Fx<'f, S, O> {
        self.0(i)
    }
}

#[derive(Clone)]
pub struct ReqFx<'f, I, S, O: Clone>(Box<dyn ReqFn<'f, I, S, O> + 'f>);

clone_trait_object!(<'f, I, S, O: Clone> ReqFn<'f, I, S, O>);

trait ReqFn<'f, I, S, O>: DynClone + Fn(I) -> Fx<'f, S, O>
where
    O: Clone + 'f,
    S: 'f,
{
}

impl<'f, I, S, O, F> ReqFn<'f, I, S, O> for F
where
    F: Fn(I) -> Fx<'f, S, O> + Clone,
    O: Clone + 'f,
    S: 'f,
{
}
