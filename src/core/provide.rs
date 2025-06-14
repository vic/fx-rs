use crate::Fx;

impl<'a, S: Clone, V: Clone> Fx<'a, S, V> {
    pub fn provide<T: Clone>(self, s: S) -> Fx<'a, T, V> {
        self.contra_map(|_t| s, |t, _s| t)
    }

    pub fn provide_part<A, B, C, F>(self, a: A, cmap: C, fmap: F) -> Fx<'a, B, V>
    where
        C: FnOnce(A, B) -> S + Clone + 'a,
        F: FnOnce(B, S) -> B + Clone + 'a,
        A: Clone + 'a,
        B: Clone + 'a,
    {
        self.contra_map(|b: B| cmap(a, b), fmap)
    }
}

impl<'a, A: Clone, B: Clone, V: Clone> Fx<'a, (A, B), V> {
    pub fn provide_left(self, a: A) -> Fx<'a, B, V> {
        self.provide_part(a, |a, b| (a, b), |_b, (_a, b)| b)
    }
}
