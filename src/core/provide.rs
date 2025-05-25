use super::{Fx, eff::Eff};

impl<'a, S: Clone, A: Clone> Fx<'a, S, A> {
    pub(crate) fn provide<T: Clone>(self, s: S) -> Fx<'a, T, A> {
        match self.0 {
            Eff::Immediate(a) => Fx::immediate(a),
            Eff::Stopped(f) => Fx::stopped(move || f().provide(s.clone())),
            Eff::Pending(f) => f(s.clone()).provide(s),
        }
    }
}
