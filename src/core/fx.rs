use crate::Fx;

impl<'f, S: Clone> Fx<'f, S, S> {
    pub fn ctx() -> Self {
        Fx::func(|s| s)
    }
}

impl<'f, S, V: Clone> Fx<'f, S, V> {
    pub fn func<F>(mut f: F) -> Fx<'f, S, V>
    where
        F: FnMut(S) -> V + Clone + 'f,
    {
        Fx::pending(move |s: S| Fx::immediate(f(s)))
    }
}
