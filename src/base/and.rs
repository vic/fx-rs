use crate::{And, Fx, Nil};

impl<'a, S, V: Clone> Fx<'a, S, V> {
    pub fn and_nil(self) -> Fx<'a, And<S, Nil>, V> {
        Fx::then(self, And::left, Fx::immediate)
    }
}

impl<'a, A, B, V: Clone> Fx<'a, And<A, B>, V> {
    pub fn and_swap(self) -> Fx<'a, And<B, A>, V> {
        self.then(And::<B, A>::swap, Fx::immediate)
    }

    pub fn and_flat(e: Fx<'a, A, Fx<'a, B, V>>) -> Self {
        e.flat_map(|v| v)
    }

    pub fn and_nest(self) -> Fx<'a, A, Fx<'a, B, V>>
    where
        A: Copy,
        B: Copy,
    {
        Fx::halted()
        // let e = self;
        // Fx::pending(move |a: A| Fx::immediate(e.provide_left(a)))
    }
}
