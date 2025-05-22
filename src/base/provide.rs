use crate::{Fx, Nil};

impl<'a, A: Clone, V: Clone> Fx<'a, A, V> {
    pub fn provide(self, a: A) -> Fx<'a, Nil, V> {
        self.and_nil().provide_left(a)
    }
}
