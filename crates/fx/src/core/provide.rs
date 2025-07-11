use super::super::kernel::fx::Fx;
use super::{has_put::Put, pair::Pair};

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

    pub fn update_context<T>(self, t: T) -> Fx<'a, S, V>
    where
        S: Put<T>,
        T: Clone + 'a,
    {
        self.contra_map(|p: S| p.put(t), |_, p| p)
    }
}

impl<'a, P: Clone, V: Clone> Fx<'a, P, V> {
    pub fn provide_left<A, B>(self, a: A) -> Fx<'a, B, V>
    where
        A: Clone + 'a,
        B: Clone,
        P: Pair<A, B>,
    {
        self.provide_part(a, |a, b| P::from((a, b)), |_b, p| p.snd())
    }
}
