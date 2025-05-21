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
}
