use super::Ability;

pub struct Fx<'f, S, V>(Eff<'f, S, V>);

enum Eff<'f, S, V> {
    Immediate(S, V),
    Pending(Box<dyn Ability<'f, S, S, V> + 'f>),
}

impl<V> Fx<'_, (), V> {
    pub fn eval(self) -> V {
        let mut e = self;
        loop {
            match e.0 {
                Eff::Immediate((), v) => return v,
                Eff::Pending(f) => e = f.apply(()),
            }
        }
    }
}

impl<'f, S, V> Fx<'f, S, V> {
    pub fn immediate(s: S, value: V) -> Self {
        Fx(Eff::Immediate(s, value))
    }

    pub fn pending<F>(f: F) -> Self
    where
        F: FnOnce(S) -> Self + 'f,
    {
        Fx(Eff::Pending(Box::new(f)))
    }

    pub fn adapt<T, U, C, F>(self, cmap: C, fmap: F) -> Fx<'f, T, U>
    where
        S: 'f,
        V: 'f,
        C: FnOnce(T) -> S + Clone + 'f,
        F: FnOnce(T, S, V) -> Fx<'f, T, U> + Clone + 'f,
    {
        Fx::pending(|t: T| match self.0 {
            Eff::Immediate(s, v) => fmap(t, s, v),
            Eff::Pending(f) => f.apply(cmap.clone()(t)).adapt(cmap, fmap),
        })
    }
}
