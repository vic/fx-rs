use super::Fx;
use dyn_clone::{DynClone, clone_trait_object};

pub enum Eff<'f, S: 'f, V: Clone + 'f> {
    Immediate(V),
    Pending(Continue<'f, S, V>),
    Stopped(Start<'f, S, V>),
}

pub trait ContinueFn<'f, S: 'f, V: Clone + 'f>: DynClone + Fn(S) -> Fx<'f, S, V> + 'f {}

impl<'f, S: 'f, V: Clone + 'f, F> ContinueFn<'f, S, V> for F where
    F: Fn(S) -> Fx<'f, S, V> + Clone + 'f
{
}

clone_trait_object!(<'f, S: 'f, V: Clone + 'f> ContinueFn<'f, S, V>);

type Continue<'f, S, V> = Box<dyn ContinueFn<'f, S, V>>;

pub trait StartFn<'f, S: 'f, V: Clone + 'f>: DynClone + Fn() -> Fx<'f, S, V> + 'f {}

impl<'f, S: 'f, V: Clone + 'f, F> StartFn<'f, S, V> for F where F: Fn() -> Fx<'f, S, V> + Clone + 'f {}

clone_trait_object!(<'f, S: 'f, V: Clone + 'f> StartFn<'f, S, V>);
type Start<'f, S, V> = Box<dyn StartFn<'f, S, V>>;

impl<'f, S: 'f, V: Clone + 'f> Clone for Eff<'f, S, V> {
    fn clone(&self) -> Self {
        match self {
            Eff::Immediate(v) => Eff::Immediate(v.clone()),
            Eff::Pending(f) => Eff::Pending(f.clone()),
            Eff::Stopped(f) => Eff::Stopped(f.clone()),
        }
    }
}
