use super::Fx;

/// An `Ability` is a function `I -> Fx<S, O>`.
pub trait Ability<'f, I, S, O> {
    fn apply(self: Box<Self>, i: I) -> Fx<'f, S, O>;
}

impl<'f, I, S, O, F> Ability<'f, I, S, O> for F
where
    F: FnOnce(I) -> Fx<'f, S, O> + 'f,
{
    fn apply(self: Box<Self>, i: I) -> Fx<'f, S, O> {
        (*self)(i)
    }
}

// Helper to create boxed abilities
pub fn ability<'f, I, S, O, F>(f: F) -> Box<dyn Ability<'f, I, S, O> + 'f>
where
    F: FnOnce(I) -> Fx<'f, S, O> + 'f,
{
    Box::new(f)
}
