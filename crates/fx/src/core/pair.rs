pub trait Pair<A, B>
where
    Self: From<(A, B)> + Into<(A, B)> + Clone,
{
    fn fst(self) -> A {
        self.into().0
    }

    fn snd(self) -> B {
        self.into().1
    }

    fn fwd<Q>(self) -> Q
    where
        Q: Pair<A, B>,
    {
        let (a, b) = self.into();
        Q::from((a, b))
    }

    fn bwd<Q>(self) -> Q
    where
        Q: Pair<B, A>,
    {
        let (a, b) = self.into();
        Q::from((b, a))
    }
}

impl<A: Clone, B: Clone> Pair<A, B> for (A, B) {}
