use crate::core::has_put::{Has, HasPut, Put};
use crate::kernel::fx::Fx;

use super::pair::Pair;
use super::state::State;

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

    // effect sequencing over HasPut requirements.
    pub fn lift_map<R, U, P, F>(self, f: F) -> Fx<'f, P, U>
    where
        U: Clone + 'f,
        R: Clone + 'f,
        P: HasPut<S, P> + HasPut<R, P> + Put<S, P> + Put<R, P> + Clone + 'f,
        F: FnOnce(V) -> Fx<'f, R, U> + Clone + 'f,
    {
        self.lift().map_m(|v| f(v).lift())
    }

    pub fn lift<T>(self) -> Fx<'f, T, V>
    where
        T: HasPut<S, T> + Put<S, T> + Clone + 'f,
        S: Clone + 'f,
        V: Clone + 'f,
    {
        self.contra_map(|t: T| t.get().clone(), |t, s| t.put(s))
    }

    pub fn zip<V2>(self, other: Fx<'f, S, V2>) -> Fx<'f, S, (V, V2)>
    where
        S: Clone + 'f,
        V: Clone + 'f,
        V2: Clone + 'f,
    {
        self.map_m(move |v| other.clone().map(move |v2| (v.clone(), v2)))
    }

    pub fn zip_left<V2>(self, other: Fx<'f, S, V2>) -> Fx<'f, S, V>
    where
        S: Clone + 'f,
        V: Clone + 'f,
        V2: Clone + 'f,
    {
        self.map_m(move |v| other.clone().map(move |_| v.clone()))
    }

    pub fn zip_right<V2>(self, other: Fx<'f, S, V2>) -> Fx<'f, S, V2>
    where
        S: Clone + 'f,
        V: Clone + 'f,
        V2: Clone + 'f,
    {
        self.map_m(move |_| other.clone())
    }
}

impl<'f, S: Clone> Fx<'f, S, ()> {
    pub fn has_pending<X, V, F>(f: F) -> Fx<'f, S, V>
    where
        S: Has<X> + Clone + 'f,
        V: Clone,
        F: FnOnce(X) -> Fx<'f, S, V> + Clone + 'f,
        X: Clone,
    {
        Fx::pending(move |ctx: S| {
            let x = Has::get(ctx.clone());
            f(x)
        })
    }
}
