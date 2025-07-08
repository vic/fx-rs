use super::Ability;

#[derive(Clone)]
pub struct Fx<'f, S: Clone, V: Clone>(Eff<'f, S, V>);

#[derive(Clone)]
enum Eff<'f, S: Clone, V: Clone> {
    Immediate(S, V),
    Pending(Box<dyn Ability<'f, S, S, V> + 'f>),
}

impl<V: Clone> Fx<'_, (), V> {
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

impl<'f, S: Clone, V: Clone> Fx<'f, S, V> {
    pub fn immediate(s: S, value: V) -> Self {
        Fx(Eff::Immediate(s, value))
    }

    pub fn pending<A>(a: A) -> Self
    where
        Box<dyn Ability<'f, S, S, V> + 'f>: From<A>
    {
        Fx(Eff::Pending(a.into()))
    }

    pub fn adapt<T, U, C, F>(self, cmap: C, fmap: F) -> Fx<'f, T, U>
    where
        T: Clone,
        S: Clone,
        U: Clone,
        C: FnOnce(T) -> S + Clone + 'f,
        F: FnOnce(T, S, V) -> Fx<'f, T, U> + Clone + 'f,
    {
        let cmap = cmap.clone();
        let fmap = fmap.clone();
        Fx::pending(move |t: T| match self.0 {
            Eff::Immediate(s, v) => fmap.clone()(t, s, v),
            Eff::Pending(f) => f.apply(cmap.clone()(t)).adapt(cmap.clone(), fmap.clone()),
        })
    }
}
