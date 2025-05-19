use crate::Fx;

impl<'f, F, O> Fx<'f, F, O> {
    pub fn apply<I>(i: I) -> Self
    where
        I: Copy + 'f,
        F: Fn(I) -> O,
    {
        Fx::map(Fx::ctx(), move |f: F| f(i))
    }
}
