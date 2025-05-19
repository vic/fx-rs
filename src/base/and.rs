use crate::{And, Fx, Nil};

impl<'a, S, V> Fx<'a, S, V> {
    pub fn and_nil(self) -> Fx<'a, And<S, Nil>, V> {
        Fx::then(self, And::left, Fx::immediate)
    }
}

impl<'a, A, B, V> Fx<'a, And<A, B>, V> {
    pub fn and_swap(self) -> Fx<'a, And<B, A>, V> {
        self.then(And::<B, A>::swap, Fx::immediate)
    }
}
