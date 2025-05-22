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

impl<'f, S: Clone> Fx<'f, S, S> {
    pub fn ctx() -> Self {
        Fx::func(|s| s)
    }
}

impl<'f, S, V: Clone> Fx<'f, S, V> {
    pub fn func<F>(f: F) -> Fx<'f, S, V>
    where
        F: Fn(S) -> V + Clone + 'f,
    {
        Fx::pending(move |s: S| Fx::immediate(f(s)))
    }

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
}
