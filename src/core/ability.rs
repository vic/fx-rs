use crate::{Ability, Fx, State};

use super::handler::Handler;

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
        State::get().flat_map(|f: Self| f.apply(i))
    }

    pub fn handler<B, V>(self) -> Handler<'f, (Self, B), B, V, V>
    where
        B: Clone,
        V: Clone,
    {
        Handler::new(|e: Fx<'f, (Self, B), V>| e.provide_left(self))
    }

    pub fn imap<Y, F>(self, imap: F) -> Ability<'f, Y, S, O>
    where
        Y: Clone + 'f,
        F: FnOnce(Y) -> I + Clone + 'f,
    {
        Ability::new(|y: Y| self.apply(imap(y)))
    }

    pub fn hmap<T, U, H>(self, h: H) -> Ability<'f, I, T, U>
    where
        T: Clone + 'f,
        U: Clone + 'f,
        H: FnOnce(Fx<'f, S, O>) -> Fx<'f, T, U> + Clone + 'f,
    {
        Ability::new(|i: I| h(self.apply(i)))
    }
}
