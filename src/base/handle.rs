use crate::{And, Fx, Handler};

impl<'f, A, V: Clone> Fx<'f, A, V> {
    pub fn handle<B, U: Clone>(self, handler: Handler<'f, A, B, V, U>) -> Fx<'f, B, U> {
        handler.handle(self)
    }
}

impl<'f, A: Clone, B: Clone, U: Clone, V: Clone> Handler<'f, A, B, U, V> {
    pub fn on_left<S: Clone>(self) -> Handler<'f, And<A, S>, And<B, S>, U, V> {
        Handler::new(move |e| e.and_swap().handle(self.clone().on_right()).and_swap())
    }

    pub fn on_right<S: Clone>(self) -> Handler<'f, And<S, A>, And<S, B>, U, V> {
        Handler::new(move |e| e.and_nest().handle(self.clone().on_value()).and_flat())
    }

    pub fn on_value<S: Clone>(self) -> Handler<'f, S, S, Fx<'f, A, U>, Fx<'f, B, V>> {
        Handler::new(move |e: Fx<'f, S, Fx<'f, A, U>>| {
            e.flat_map(|i: Fx<'f, A, U>| Fx::func(move |h: Self| h.handle(i.clone())))
                .and_swap()
                .provide_left(self.clone())
        })
    }
}
