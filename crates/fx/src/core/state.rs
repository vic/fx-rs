use std::marker::PhantomData;

use super::field::Field;
use crate::{Fx, Lens, Pair};

pub struct State<'f, S: Clone>(PhantomData<&'f S>);
impl<'f, S: Clone> State<'f, S> {
    pub fn get<T>() -> Fx<'f, S, T>
    where
        S: Field<T> + Clone + 'f,
        T: Clone + 'f,
    {
        Fx::pending(|s: S| Fx::immediate(s.clone(), s.field().clone()))
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

    pub fn update<T, P, F>(f: F) -> Fx<'f, P, S>
    where
        T: Clone + 'f,
        F: FnOnce(S) -> Fx<'f, T, S> + Clone + 'f,
        P: Pair<S, T>,
    {
        Self::get().flat_map(f).via(Lens::left().zoom_in(Self::set))
    }
}
