use crate::Fx;

impl<'f> Fx<'f, (), ()> {
    pub fn nil() -> Fx<'f, (), ()> {
        Fx::immediate(())
    }
}

impl<'f, V> Fx<'f, (), V>
where
    V: Clone,
{
    pub fn pure(v: V) -> Fx<'f, (), V> {
        Fx::immediate(v)
    }
}

impl<'f, S, V> Fx<'f, S, V>
where
    S: Clone,
    V: Clone,
{
    // pub fn resume(self) -> Self {
    //     self.start(|e| e)
    // }

    // pub fn restart<F>(self, f: F) -> Self
    // where
    //     F: FnOnce() -> Self + Clone + 'f,
    // {
    //     self.start(move |_| f())
    // }

    // pub fn halted() -> Fx<'f, S, V> {
    //     Fx::stopped(|| Fx::func(|_: S| -> V { panic!("tried to use halted effect.") }))
    // }

    pub fn via<F, T, U>(self, f: F) -> Fx<'f, T, U>
    where
        F: FnOnce(Self) -> Fx<'f, T, U>,
        T: Clone,
        U: Clone,
    {
        f(self)
    }
}

impl<'f, S, V, F> From<F> for Fx<'f, S, V>
where
    S: Clone,
    V: Clone,
    F: FnOnce(S) -> V + Clone + 'f,
{
    fn from(f: F) -> Self {
        Fx::func(f)
    }
}
