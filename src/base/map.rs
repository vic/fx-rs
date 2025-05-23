use crate::Fx;

impl<'f, S, V: Clone> Fx<'f, S, V> {
    pub fn contra_map<T, F>(self, cmap: F) -> Fx<'f, T, V>
    where
        F: Fn(T) -> S + Clone + 'f,
    {
        self.then(cmap, Fx::immediate)
    }

    pub fn map<F, U>(self, f: F) -> Fx<'f, S, U>
    where
        U: Clone,
        F: Fn(V) -> U + Clone + 'f,
    {
        self.map_m(move |v| Fx::immediate(f(v)))
    }

    pub fn map_m<F, U>(self, f: F) -> Fx<'f, S, U>
    where
        U: Clone,
        F: Fn(V) -> Fx<'f, S, U> + Clone + 'f,
    {
        self.then(|c| c, f)
    }

    pub fn flat_map<F, T, U>(self, f: F) -> Fx<'f, (S, T), U>
    where
        U: Clone,
        F: Fn(V) -> Fx<'f, T, U> + Clone + 'f,
    {
        self.then(
            |n: (S, T)| n.0,
            move |v| f(v).then(|n: (S, T)| n.1, Fx::immediate),
        )
    }
}
