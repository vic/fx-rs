use crate::Fx;

impl<'a, S: Clone, V: Clone> Fx<'a, S, V> {
    pub fn provide_part<A, B, C, F>(self, a: A, cmap: C, fmap: F) -> Fx<'a, B, V>
    where
        C: FnOnce(A, B) -> S + Clone + 'a,
        F: FnOnce(S) -> B + Clone + 'a,
        A: Clone + 'a,
        B: Clone + 'a,
    {
        self.cmap(|b: B| cmap(a, b), fmap)
    }
}

impl<'a, A: Clone, B: Clone, V: Clone> Fx<'a, (A, B), V> {
    pub fn provide_left(self, a: A) -> Fx<'a, B, V> {
        self.provide_part(a, |a, b| (a, b), |(_a, b)| b)
    }
}
