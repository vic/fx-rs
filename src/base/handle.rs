use crate::{And, Fx};
use dyn_clone::{DynClone, clone_trait_object};

impl<'f, I, O: Clone> Fx<'f, I, O> {
    pub fn apply<F>(i: I) -> Fx<'f, F, O>
    where
        I: Copy + 'f,
        F: Fn(I) -> O + Clone,
    {
        Fx::map(Fx::ctx(), move |f: F| f(i))
    }

    pub fn suspend<F, B>(i: I) -> Fx<'f, And<F, B>, O>
    where
        I: Copy + 'f,
        B: 'f,
        F: Fn(I) -> Fx<'f, B, O> + Clone,
    {
        Fx::and_flat(Fx::apply(i))
    }

    pub fn handler<F, B>(f: F) -> Handler<'f, And<F, B>, B, O, O>
    where
        B: Copy + 'f,
        F: Fn(I) -> Fx<'f, B, O> + Copy + 'f,
    {
        Handler::new(move |e| e.provide_left(f))
    }

    pub fn handle<F, B>(i: I) -> Fx<'f, And<Handler<'f, And<F, B>, B, O, O>, B>, O>
    where
        F: Fn(I) -> Fx<'f, B, O> + Clone + 'f,
        B: Clone + 'f,
        I: Copy,
    {
        Fx::ctx().flat_map(move |h: Handler<'f, And<F, B>, B, O, O>| h.handle(Fx::suspend(i)))
    }
}

#[derive(Clone)]
pub struct Handler<'f, A, B, U: Clone, V: Clone>(Box<dyn HandlerFn<'f, A, B, U, V> + 'f>);
impl<'f, A, B, U: Clone, V: Clone> Handler<'f, A, B, U, V> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(Fx<'f, A, U>) -> Fx<'f, B, V> + Clone + 'f,
    {
        Handler(Box::new(f))
    }

    pub fn handle(&self, e: Fx<'f, A, U>) -> Fx<'f, B, V> {
        self.0(e)
    }
}

clone_trait_object!(<'f, A, B, U:Clone, V:Clone> HandlerFn<'f, A, B, U, V>);
trait HandlerFn<'f, A, B, U, V>: DynClone + Fn(Fx<'f, A, U>) -> Fx<'f, B, V> + 'f
where
    V: Clone + 'f,
    U: Clone + 'f,
    A: 'f,
    B: 'f,
{
}
impl<'f, A, B, U: Clone, V: Clone, F> HandlerFn<'f, A, B, U, V> for F
where
    F: Fn(Fx<'f, A, U>) -> Fx<'f, B, V> + Clone + 'f,
    V: Clone + 'f,
    U: Clone + 'f,
    A: 'f,
    B: 'f,
{
}
