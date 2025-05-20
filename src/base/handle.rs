use crate::{And, Fx};

impl<'f, I, O: Clone> Fx<'f, I, O> {
    pub fn apply<F>(i: I) -> Fx<'f, F, O>
    where
        I: Copy + 'f,
        F: Fn(I) -> O + Clone,
    {
        Fx::map(Fx::ctx(), move |f: F| f(i))
    }

    pub fn suspend<F, B>(i: I) -> Fx<'f, And<F, B>, O>
    where
        I: Copy + 'f,
        B: 'f,
        F: Fn(I) -> Fx<'f, B, O> + Clone,
    {
        Fx::and_flat(Fx::apply(i))
    }

    pub fn handler<F, B>(f: F) -> impl Fn(Fx<'f, And<F, B>, O>) -> Fx<'f, B, O>
    where
        B: Copy + 'f,
        F: Fn(I) -> Fx<'f, B, O> + Copy,
    {
        move |e| e.provide_left(f)
    }

    /*

    func Handle[F ~func(Fx[And[A, B], O]) Fx[B, O], A ~func(I) Fx[B, O], B, I, O any](i I) Fx[And[F, B], O] {
        return Suspend[F](Suspend[A](i))
    }
         */
}
