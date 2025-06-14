use std::marker::PhantomData;

use crate::{Fx, Lens};

pub struct State<'f, S: Clone>(PhantomData<&'f S>);
impl<'f, S: Clone> State<'f, S> {
    pub fn get() -> Fx<'f, S, S> {
        Fx::pending(Fx::value)
    }

    pub fn set(s: S) -> Fx<'f, S, S> {
        Fx::immediate(s.clone(), s)
    }

    pub fn map<F>(f: F) -> Fx<'f, S, S>
    where
        F: FnOnce(S) -> S + Clone + 'f,
    {
        Self::map_m(|s| Fx::value(f(s)))
    }

    pub fn map_m<F>(f: F) -> Fx<'f, S, S>
    where
        F: FnOnce(S) -> Fx<'f, S, S> + Clone + 'f,
    {
        Self::get().map_m(f).map_m(State::set)
    }

    pub fn update<T, F>(f: F) -> Fx<'f, (S, T), S>
    where
        T: Clone,
        F: FnOnce(S) -> Fx<'f, T, S> + Clone + 'f,
    {
        Self::get().flat_map(f).via(Lens::left().zoom_in(Self::set))
    }
}
