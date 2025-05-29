use crate::{Fx, Nil};

impl<'a, S, V: Clone> Fx<'a, S, V> {
    pub fn and_nil(self) -> Fx<'a, (S, Nil), V> {
        self.contra_map(|(s, _)| s)
    }

    pub fn into_env(self) -> Fx<'a, (S, V), (V, V)> {
        self.flat_map(|v| Fx::func(|n: (V, V)| n).provide_left(v))
    }

    pub fn from_env(self) -> Fx<'a, S, (S, V)>
    where
        S: Clone,
    {
        self.map_m(|v| Fx::func(|s: (V, S)| (s.1, s.0)).provide_left(v))
    }
}

impl<'a, S: Clone, V: Clone> Fx<'a, (S, S), V> {
    pub fn and_collapse(self) -> Fx<'a, S, V> {
        self.contra_map(|s: S| (s.clone(), s))
    }
}

impl<'a, S: Clone, B: Clone, V: Clone> Fx<'a, ((S, B), S), V> {
    pub fn and_collapse_left(self) -> Fx<'a, (S, B), V> {
        self.contra_map(|(s, b): (S, B)| ((s.clone(), b), s))
    }
}

impl<'a, A: Clone, B: Clone, C: Clone, V: Clone> Fx<'a, (A, (B, C)), V> {
    pub fn and_rotate(self) -> Fx<'a, (C, (A, B)), V> {
        self.contra_map(|(c, (a, b))| (a, (b, c)))
    }
}

impl<'a, A, B, V: Clone> Fx<'a, (A, B), V> {
    pub fn and_swap(self) -> Fx<'a, (B, A), V> {
        self.contra_map(|(b, a)| (a, b))
    }

    pub fn and_nest(self) -> Fx<'a, A, Fx<'a, B, V>>
    where
        A: Clone,
        B: Clone,
    {
        Fx::func(move |a: A| self.clone().provide_left(a))
    }
}

impl<'a, A, B, V: Clone> Fx<'a, A, Fx<'a, B, V>> {
    pub fn and_flat(self) -> Fx<'a, (A, B), V> {
        self.flat_map(|v| v)
    }
}
