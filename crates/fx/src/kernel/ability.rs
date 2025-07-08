use dyn_clone::{DynClone, clone_trait_object};

use super::fx::Fx;

// Ability trait: represents a function I -> Fx<S, O>
pub trait Ability<'f, I, S, O>: DynClone + 'f
where
    I: Clone + 'f,
    S: Clone + 'f,
    O: Clone + 'f,
{
    fn apply(&self, i: I) -> Fx<'f, S, O>;
}

clone_trait_object!(<'f, I: Clone + 'f, S: Clone + 'f, O: Clone + 'f> Ability<'f, I, S, O>);

// Blanket impl for closures
impl<'f, I, S, O, F> Ability<'f, I, S, O> for F
where
    I: Clone + 'f,
    S: Clone + 'f,
    O: Clone + 'f,
    F: FnOnce(I) -> Fx<'f, S, O> + Clone + 'f,
{
    fn apply(&self, i: I) -> Fx<'f, S, O> {
        (self.clone())(i)
    }
}
