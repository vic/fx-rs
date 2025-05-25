use crate::Fx;

impl<'a, S: Clone, V: Clone> Fx<'a, S, V> {
    pub fn provide_part<A, B, F>(self, a: A, cmap: F) -> Fx<'a, B, V>
    where
        F: Fn(A, B) -> S + Clone + 'a,
        A: Clone + 'a,
        B: Clone + 'a,
    {
        Fx::pending(move |b: B| self.clone().provide(cmap(a.clone(), b)))
    }
}
