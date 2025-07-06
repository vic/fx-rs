use crate::Fx;

use super::{Pair, State};

impl<'f, V: Clone> Fx<'f, (), V> {
    pub fn pure(value: V) -> Self {
        Fx::immediate((), value)
    }
}

impl<'f, S: Clone, V: Clone> Fx<'f, S, V> {
    pub fn value(value: V) -> Self {
        Fx::pending(|s: S| Fx::immediate(s, value))
    }

    pub fn func<F>(f: F) -> Self
    where
        F: FnOnce(S) -> V + Clone + 'f,
    {
        State::get().map(f)
    }

    pub fn map_m<U, F>(self, f: F) -> Fx<'f, S, U>
    where
        U: Clone,
        F: FnOnce(V) -> Fx<'f, S, U> + Clone + 'f,
    {
        self.adapt(|s| s, |_s, s, v| f(v).contra_map(|_| s, |_s, s| s))
    }

    pub fn map<U, F>(self, f: F) -> Fx<'f, S, U>
    where
        U: Clone,
        F: FnOnce(V) -> U + Clone + 'f,
    {
        self.map_m(|v| Fx::value(f(v)))
    }

    pub fn flat_map<R, U, P, F>(self, f: F) -> Fx<'f, P, U>
    where
        U: Clone,
        V: Clone,
        R: Clone + 'f,
        P: Pair<S, R>,
        F: FnOnce(V) -> Fx<'f, R, U> + Clone + 'f,
    {
        self.adapt(
            |p: P| p.fst(),
            |_, s, v| {
                f(v).adapt(
                    |p: P| p.snd(),
                    |_, r, u| Fx::value(u).contra_map(|_| P::from((s, r)), |_, p| p),
                )
            },
        )
    }

    pub fn then<U>(self, e: Fx<'f, S, U>) -> Fx<'f, S, U>
    where
        U: Clone,
    {
        self.map_m(|_| e)
    }

    pub fn and_then<T, U, P>(self, e: Fx<'f, T, U>) -> Fx<'f, P, U>
    where
        T: Clone,
        U: Clone,
        P: Pair<S, T>,
    {
        self.flat_map(|_| e)
    }

    pub fn contra_map<Outer, Getter, Setter>(
        self,
        getter: Getter,
        setter: Setter,
    ) -> Fx<'f, Outer, V>
    where
        Outer: Clone,
        Getter: FnOnce(Outer) -> S + Clone + 'f,
        Setter: FnOnce(Outer, S) -> Outer + Clone + 'f,
    {
        self.adapt(
            |t: Outer| getter(t),
            |t, s, v| Fx::immediate(setter(t, s), v),
        )
    }
}
