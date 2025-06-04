use crate::Fx;

impl<'a, S: Clone, V: Clone> Fx<'a, S, V> {
    pub fn and_nil(self) -> Fx<'a, (S, ()), V> {
        self.contra_map(|(s, _)| s, |_, s| (s, ()))
    }
}

impl<'a, S: Clone, V: Clone> Fx<'a, (S, S), V> {
    pub fn and_collapse(self) -> Fx<'a, S, V> {
        self.contra_map(|s: S| (s.clone(), s), |_, (_, s)| s)
    }
}

impl<'a, S: Clone, B: Clone, V: Clone> Fx<'a, ((S, B), S), V> {
    pub fn and_collapse_left(self) -> Fx<'a, (S, B), V> {
        self.contra_map(|(s, b): (S, B)| ((s.clone(), b), s), |_, (sb, _)| sb)
    }
}

impl<'a, A: Clone, B: Clone, C: Clone, V: Clone> Fx<'a, (A, (B, C)), V> {
    pub fn and_rotate(self) -> Fx<'a, (C, (A, B)), V> {
        self.contra_map(|(c, (a, b))| (a, (b, c)), |_, (a, (b, c))| (c, (a, b)))
    }
}

impl<'a, A: Clone, B: Clone, V: Clone> Fx<'a, (A, B), V> {
    pub fn and_swap(self) -> Fx<'a, (B, A), V> {
        self.contra_map(|(b, a)| (a, b), |_, (a, b)| (b, a))
    }

    pub fn and_nest(self) -> Fx<'a, A, Fx<'a, B, V>> {
        Fx::func(move |a: A| self.clone().provide_left(a))
    }
}

impl<'a, A: Clone, B: Clone, V: Clone> Fx<'a, A, Fx<'a, B, V>> {
    pub fn and_flat(self) -> Fx<'a, (A, B), V> {
        self.flat_map(|v| v)
    }
}
