use crate::{Fx, Nil};

impl<'f> Fx<'f, Nil, Nil> {
    pub fn nil() -> Fx<'f, Nil, Nil> {
        Fx::immediate(Nil)
    }
}

impl<'f, V: Clone> Fx<'f, Nil, V> {
    pub fn pure(v: V) -> Fx<'f, Nil, V> {
        Fx::immediate(v)
    }
}

impl<'f, I, O: Clone> Fx<'f, I, O> {
    pub fn apply<F>(i: I) -> Fx<'f, F, O>
    where
        I: Clone + 'f,
        F: Fn(I) -> O + Clone,
    {
        Fx::ctx().map(move |f: F| f(i.clone()))
    }
}

impl<'f, S, V: Clone> Fx<'f, S, V> {
    pub fn resume(self) -> Self {
        self.start(|e| e)
    }

    pub fn restart<F>(self, f: F) -> Self
    where
        F: Fn() -> Self + Clone + 'f,
    {
        self.start(move |_| f())
    }

    pub fn halted() -> Fx<'f, S, V> {
        Fx::stopped(|| Fx::func(|_: S| -> V { panic!("tried to use halted effect.") }))
    }

    pub fn via<F, T, U>(self, f: F) -> Fx<'f, T, U>
    where
        F: Fn(Self) -> Fx<'f, T, U>,
        U: Clone,
    {
        f(self)
    }

    pub fn unit(self) -> Fx<'f, S, ()> {
        self.constant(())
    }

    pub fn constant<U: Clone>(self, v: U) -> Fx<'f, S, U> {
        self.map(move |_| v.clone())
    }
}

impl<'f, S, V, F> From<F> for Fx<'f, S, V>
where
    V: Clone,
    F: Fn(S) -> V + Clone + 'f,
{
    fn from(f: F) -> Self {
        Fx::func(f)
    }
}
