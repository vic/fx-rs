use crate::{core::pair::Pair, kernel::fx::Fx};

impl<'a, S: Clone, V: Clone> Fx<'a, S, V> {
    pub fn and_nil<P: Pair<S, ()>>(self) -> Fx<'a, P, V> {
        self.contra_map(|p: P| p.fst(), |_, s| P::from((s, ())))
    }
}

impl<'a, P: Clone, V: Clone> Fx<'a, P, V> {
    pub fn and_collapse<S: Clone>(self) -> Fx<'a, S, V>
    where
        P: Pair<S, S>,
    {
        self.contra_map(|s: S| P::from((s.clone(), s)), |_, p| p.snd())
    }
}

impl<'a, SBS: Clone, V: Clone> Fx<'a, SBS, V> {
    pub fn and_collapse_left<S, B, SB>(self) -> Fx<'a, SB, V>
    where
        S: Clone,
        B: Clone,
        SB: Pair<S, B>,
        SBS: Pair<SB, S>,
    {
        self.contra_map(
            |sb: SB| {
                let (s, b) = sb.into();
                SBS::from((SB::from((s.clone(), b)), s))
            },
            |_, sbs| sbs.fst(),
        )
    }
}

impl<'a, ABC: Clone, V: Clone> Fx<'a, ABC, V> {
    pub fn and_rotate<A, B, C, BC, AB, CAB>(self) -> Fx<'a, CAB, V>
    where
        A: Clone,
        B: Clone,
        C: Clone,
        BC: Pair<B, C>,
        ABC: Pair<A, BC>,
        AB: Pair<A, B>,
        CAB: Pair<C, AB>,
    {
        self.contra_map(
            |cab: CAB| {
                let (c, ab) = cab.into();
                let (a, b) = ab.into();
                ABC::from((a, BC::from((b, c))))
            },
            |_, abc: ABC| {
                let (a, bc) = abc.into();
                let (b, c) = bc.into();
                CAB::from((c, AB::from((a, b))))
            },
        )
    }
}

impl<'a, P: Clone, V: Clone> Fx<'a, P, V> {
    pub fn and_swap<A, B, Q>(self) -> Fx<'a, Q, V>
    where
        A: Clone,
        B: Clone,
        P: Pair<A, B>,
        Q: Pair<B, A>,
    {
        self.contra_map(|q: Q| q.bwd(), |_, p| p.bwd())
    }

    pub fn and_nest<A, B>(self) -> Fx<'a, A, Fx<'a, B, V>>
    where
        A: Clone,
        B: Clone,
        P: Pair<A, B>,
    {
        Fx::func(|a: A| self.provide_left(a))
    }
}

impl<'a, A: Clone, B: Clone, V: Clone> Fx<'a, A, Fx<'a, B, V>> {
    pub fn and_flat<P>(self) -> Fx<'a, P, V>
    where
        P: Pair<A, B>,
    {
        self.flat_map(|v| v)
    }
}
