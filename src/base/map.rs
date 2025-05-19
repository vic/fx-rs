use crate::{And, Fx};

impl<'f, S, V> Fx<'f, S, V> {
    pub fn contra_map<T, F>(self, cmap: F) -> Fx<'f, T, V>
    where
        F: Fn(T) -> S + Copy + 'f,
    {
        self.then(cmap, Fx::immediate)
    }

    pub fn map<F, U>(self, f: F) -> Fx<'f, S, U>
    where
        U: 'f,
        F: Fn(V) -> U + Copy + 'f,
    {
        self.map_m(move |v| Fx::immediate(f(v)))
    }

    pub fn map_m<F, U>(self, f: F) -> Fx<'f, S, U>
    where
        F: Fn(V) -> Fx<'f, S, U> + Copy + 'f,
    {
        self.then(|c| c, f)
    }

    pub fn flat_map<F, T, U>(self, f: F) -> Fx<'f, And<S, T>, U>
    where
        F: Fn(V) -> Fx<'f, T, U> + Copy + 'f,
    {
        self.then(And::left, move |v| f(v).then(And::right, Fx::immediate))
    }
}
