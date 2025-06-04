use super::Fx;
use dyn_clone::{DynClone, clone_trait_object};

#[derive(Clone)]
pub(super) enum Eff<'f, S: Clone, V: Clone> {
    Immediate(S, V),
    Pending(Continue<'f, S, V>),
}

pub(super) type Continue<'f, S, V> = Box<dyn ContinueFn<'f, S, V>>;

clone_trait_object!(<'f, S: Clone + 'f, V: Clone + 'f> ContinueFn<'f, S, V>);
pub(super) trait ContinueFn<'f, S: Clone + 'f, V: Clone + 'f>:
    DynClone + FnOnce(S) -> Fx<'f, S, V> + 'f
{
}

impl<'f, S: Clone + 'f, V: Clone + 'f, F> ContinueFn<'f, S, V> for F where
    F: FnOnce(S) -> Fx<'f, S, V> + Clone + 'f
{
}
