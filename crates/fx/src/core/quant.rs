// Quantified Effects core trait and combinator for fx-rs

use crate::Fx;

/// Trait for universally quantified effectful computations.
pub trait Quantify<'f, S: Clone, T: Clone> {
    fn quantify<F>(f: F) -> Fx<'f, S, T>
    where
        F: for<'a> FnOnce(Fx<'a, S, T>) -> Fx<'a, S, T> + Clone + 'f;
}

// Blanket impl for Fx: allows .quantify() as a combinator
impl<'f, S: Clone, T: Clone> Fx<'f, S, T> {
    pub fn quantify<F>(self, f: F) -> Fx<'f, S, T>
    where
        F: for<'a> FnOnce(Fx<'a, S, T>) -> Fx<'a, S, T> + Clone + 'f,
    {
        f(self)
    }
}
