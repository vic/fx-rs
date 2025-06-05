use super::{Ctx, Fx};

impl<'f, S: Clone, V: Clone> Fx<'f, S, V> {
    pub fn provide(self, s: S) -> Fx<'f, (), V> {
        self.cmap(|_: ()| s, |_: S| ())
    }

    pub fn func<F>(f: F) -> Self
    where
        F: FnOnce(S) -> V + Clone + 'f,
    {
        Ctx::get().map(|s: S| f(s))
    }

    pub fn map_m<U, F>(self, f: F) -> Fx<'f, S, U>
    where
        U: Clone,
        F: FnOnce(V) -> Fx<'f, S, U> + Clone + 'f,
    {
        self.adapt(|s| s, |s, v| f(v).cmap(|_| s, |s| s))
    }

    pub fn map<U, F>(self, f: F) -> Fx<'f, S, U>
    where
        U: Clone,
        F: FnOnce(V) -> U + Clone + 'f,
    {
        self.map_m(|v| Fx::value(f(v)))
    }

    pub fn flat_map<R, U, F>(self, f: F) -> Fx<'f, (S, R), U>
    where
        U: Clone,
        V: Clone,
        R: Clone,
        F: FnOnce(V) -> Fx<'f, R, U> + Clone + 'f,
    {
        self.adapt(
            |(s, _)| s,
            |s, v| f(v).adapt(|(_, r)| r, |r, u| Fx::value(u).cmap(|_| (s, r), |x| x)),
        )
    }
}
