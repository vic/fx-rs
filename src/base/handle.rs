use crate::{And, Fx, Handler};

impl<'f, A, B, V: Clone> Fx<'f, And<A, B>, V> {
    pub fn handle_left<U: Clone>(self, handler: Handler<'f, And<A, B>, B, V, U>) -> Fx<'f, B, U> {
        handler.handle(self)
    }
}

impl<'f, I, O: Clone> Fx<'f, I, O> {
    pub fn apply<F>(i: I) -> Fx<'f, F, O>
    where
        I: Copy + 'f,
        F: Fn(I) -> O + Clone,
    {
        Fx::ctx().map(move |f: F| f(i))
    }

    // pub fn request<F, B>(i: I) -> Fx<'f, And<F, B>, O>
    // where
    //     I: Copy + 'f,
    //     B: 'f,
    //     F: Fn(I) -> Fx<'f, B, O> + Clone,
    // {
    //     Fx::ctx().flat_map(move |f: F| f(i))
    // }

    // pub fn handler<F, B>(f: F) -> Handler<'f, And<F, B>, B, O, O>
    // where
    //     B: Copy + 'f,
    //     F: Fn(I) -> Fx<'f, B, O> + Copy + 'f,
    // {
    //     Handler::new(move |e| e.provide_left(f))
    // }

    // pub fn handle<F, B>(i: I) -> Fx<'f, And<Handler<'f, And<F, B>, B, O, O>, B>, O>
    // where
    //     F: Fn(I) -> Fx<'f, B, O> + Clone + 'f,
    //     B: Clone + 'f,
    //     I: Copy,
    // {
    //     Fx::ctx().flat_map(move |h: Handler<'f, And<F, B>, B, O, O>| h.handle(Fx::request(i)))
    // }
}
