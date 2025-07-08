use crate::kernel::fx::Fx;

impl<'f, S: Clone, T: Clone> Fx<'f, S, T> {
    pub fn forall<F>(self, f: F) -> Self
    where
        F: for<'a> FnOnce(Self) -> Self + Clone + 'f,
    {
        f(self)
    }
}
