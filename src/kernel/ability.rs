use dyn_clone::{DynClone, clone_trait_object};

use super::Fx;

#[derive(Clone)]
/// An `Ability` is a function `I -> Fx<S, O>`.
pub struct Ability<'f, I, S, O>(Box<dyn AbilityFn<'f, I, S, O> + 'f>)
where
    S: Clone,
    O: Clone;

clone_trait_object!(<'f, I: Clone + 'f, S: Clone + 'f, O: Clone + 'f> AbilityFn<'f, I, S, O>);
pub trait AbilityFn<'f, I: Clone + 'f, S: Clone + 'f, O: Clone + 'f>:
    DynClone + FnOnce(I) -> Fx<'f, S, O> + 'f
{
}

impl<'f, I: Clone + 'f, S: Clone + 'f, O: Clone + 'f, F> AbilityFn<'f, I, S, O> for F where
    F: FnOnce(I) -> Fx<'f, S, O> + Clone + 'f
{
}

impl<'f, I: Clone + 'f, S: Clone + 'f, O: Clone + 'f, F> From<F> for Ability<'f, I, S, O>
where
    F: FnOnce(I) -> Fx<'f, S, O> + Clone + 'f,
{
    fn from(value: F) -> Self {
        Ability::new(value)
    }
}

impl<'f, I: Clone + 'f, S: Clone + 'f, O: Clone + 'f> From<Ability<'f, I, S, O>>
    for Box<dyn AbilityFn<'f, I, S, O> + 'f>
{
    fn from(value: Ability<'f, I, S, O>) -> Self {
        value.0
    }
}

impl<'f, I, S, O> Ability<'f, I, S, O>
where
    S: Clone,
    O: Clone,
    I: Clone,
{
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(I) -> Fx<'f, S, O> + Clone + 'f,
    {
        Self(Box::new(f))
    }

    pub fn apply(self, i: I) -> Fx<'f, S, O> {
        self.0(i)
    }

    pub fn clone_boxed(&self) -> Box<dyn FnOnce(I) -> Fx<'f, S, O> + 'f> {
        self.0.clone()
    }
}
