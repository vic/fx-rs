use std::marker::PhantomData;

use crate::{
    core::{handler::Handler, pair::Pair, state::State},
    kernel::{ability::Ability, fx::Fx},
};

#[derive(Clone)]
pub struct Abilities<'f, I, S, O>(PhantomData<&'f (I, S, O)>)
where
    I: Clone + 'f,
    S: Clone + 'f,
    O: Clone + 'f;

impl<'f, I, S, O> Abilities<'f, I, S, O>
where
    I: Clone + 'f,
    S: Clone + 'f,
    O: Clone + 'f,
{
    pub fn new<F>(f: F) -> impl Ability<'f, I, S, O> + Clone + 'f
    where
        F: FnOnce(I) -> Fx<'f, S, O> + Clone + 'f,
    {
        f
    }

    pub fn boxed(a: impl Ability<'f, I, S, O>) -> Box<dyn Ability<'f, I, S, O> + 'f> {
        Box::new(a)
    }

    pub fn request<P, A>(i: I) -> Fx<'f, P, O>
    where
        A: Ability<'f, I, S, O> + Clone,
        P: Pair<A, S>,
    {
        State::get().flat_map(|a: A| a.apply(i))
    }
}

pub trait AbilityExt<'f, I, S, O>
where
    O: Clone + 'f,
    S: Clone + 'f,
    I: Clone + 'f,
    Self: Ability<'f, I, S, O> + Clone + 'f,
{
    fn handler<B, V, P>(self) -> impl Handler<'f, P, B, V, V>
    where
        B: Clone + 'f,
        V: Clone + 'f,
        P: Pair<Self, B> + 'f,
    {
        |e: Fx<'f, P, V>| e.provide_left(self)
    }

    fn imap<Y, F>(self, imap: F) -> impl Ability<'f, Y, S, O> + Clone + 'f
    where
        Y: Clone + 'f,
        F: FnOnce(Y) -> I + Clone + 'f,
    {
        move |y: Y| self.apply(imap(y))
    }

    fn hmap<T, U, H>(self, h: H) -> impl Ability<'f, I, T, U> + Clone + 'f
    where
        T: Clone + 'f,
        U: Clone + 'f,
        H: FnOnce(Fx<'f, S, O>) -> Fx<'f, T, U> + Clone + 'f,
    {
        move |i: I| h(self.apply(i))
    }
}

impl<'f, I, S, O, A> AbilityExt<'f, I, S, O> for A
where
    I: Clone + 'f,
    S: Clone + 'f,
    O: Clone + 'f,
    A: Ability<'f, I, S, O> + Clone,
{
}

impl<'f, I, S, O> Ability<'f, I, S, O> for Box<dyn Ability<'f, I, S, O>>
where
    I: Clone + 'f,
    S: Clone + 'f,
    O: Clone + 'f,
{
    fn apply(&self, i: I) -> Fx<'f, S, O> {
        self.as_ref().apply(i)
    }
}
