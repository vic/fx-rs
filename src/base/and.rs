use crate::{And, Fx, Nil};

impl<'a, S, V: Clone> Fx<'a, S, V> {
    pub fn and_nil(self) -> Fx<'a, And<S, Nil>, V> {
        self.then(And::left, Fx::immediate)
    }

    pub fn to_env(self) -> Fx<'a, And<S, V>, ()> {
        self.flat_map(|v| Fx::func(|_: And<V, V>| ()).provide_left(v))
    }
}

impl<'a, S: Clone, V: Clone> Fx<'a, And<S, S>, V> {
    pub fn and_collapse(self) -> Fx<'a, S, V> {
        self.contra_map(|s: S| And::new(s.clone(), s))
    }
}

impl<'a, S: Clone, B: Clone, V: Clone> Fx<'a, And<And<S, B>, S>, V> {
    pub fn and_collapse_left(self) -> Fx<'a, And<S, B>, V> {
        self.contra_map(|s: And<S, B>| And::new(s.clone(), s.left()))
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
        A: Clone,
        B: Clone,
    {
        Fx::pending(move |a: A| Fx::immediate((&self).clone().provide_left(a)))
    }

    pub fn left_down(self) -> Fx<'a, And<A, B>, (A, V)>
    where
        A: Clone,
        B: Clone,
    {
        self.map_m(|v| {
            Fx::func(|n: And<V, And<A, B>>| (n.clone().right().left(), n.left())).provide_left(v)
        })
    }
}
