use crate::{And, Fx};

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

    pub fn handler<F, B>(f: F) -> Box<dyn Fn(Fx<'f, And<F, B>, O>) -> Fx<'f, B, O> + 'f>
    where
        B: Copy + 'f,
        F: Fn(I) -> Fx<'f, B, O> + Copy + 'f,
    {
        Box::new(move |e| e.provide_left(f))
    }

    pub fn handle<F, A, B>(i: I) -> Fx<'f, And<F, B>, O>
    where
        F: Fn(Fx<'f, And<A, B>, O>) -> Fx<'f, B, O> + Clone,
        A: Fn(I) -> Fx<'f, B, O> + Clone + 'f,
        B: 'f,
        I: Copy,
    {
        Fx::ctx().flat_map(move |f: F| f(Fx::suspend(i)))
    }
}
