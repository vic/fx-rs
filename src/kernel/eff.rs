use super::Fx;
use dyn_clone::{DynClone, clone_trait_object};

pub(super) enum Eff<'f, S, V: Clone> {
    Immediate(V),
    Pending(Continue<'f, S, V>),
    Stopped(Start<'f, S, V>),
}

pub(super) type Continue<'f, S, V> = Box<dyn ContinueFn<'f, S, V>>;
pub(super) type Start<'f, S, V> = Box<dyn StartFn<'f, S, V>>;

clone_trait_object!(<'f, S: 'f, V: Clone + 'f> ContinueFn<'f, S, V>);
pub(super) trait ContinueFn<'f, S: 'f, V: Clone + 'f>:
    DynClone + FnOnce(S) -> Fx<'f, S, V> + 'f
{
}

impl<'f, S: 'f, V: Clone + 'f, F> ContinueFn<'f, S, V> for F where
    F: FnOnce(S) -> Fx<'f, S, V> + Clone + 'f
{
}

clone_trait_object!(<'f, S: 'f, V: Clone + 'f> StartFn<'f, S, V>);
pub(super) trait StartFn<'f, S: 'f, V: Clone + 'f>:
    DynClone + FnOnce() -> Fx<'f, S, V> + 'f
{
}

impl<'f, S: 'f, V: Clone + 'f, F> StartFn<'f, S, V> for F where
    F: FnOnce() -> Fx<'f, S, V> + Clone + 'f
{
}

impl<'f, S: 'f, V: Clone + 'f> Clone for Eff<'f, S, V> {
    fn clone(&self) -> Self {
        match self {
            Eff::Immediate(v) => Eff::Immediate(v.clone()),
            Eff::Pending(f) => Eff::Pending(f.clone()),
            Eff::Stopped(f) => Eff::Stopped(f.clone()),
        }
    }
}
