use super::eff::Eff;

#[derive(Clone)]
pub struct Fx<'f, S: Clone, V: Clone>(Eff<'f, S, V>);

impl<V: Clone> Fx<'_, (), V> {
    pub fn eval(self) -> V {
        let mut e = self;
        loop {
            match e.0 {
                Eff::Immediate((), v) => return v,
                Eff::Pending(f) => e = f(()),
            }
        }
    }
}

pub struct Ctx;
impl Ctx {
    pub fn get<'f, S: Clone>() -> Fx<'f, S, S> {
        Fx::pending(Fx::value)
    }

    pub fn set<'f, S: Clone>(s: S) -> Fx<'f, S, S> {
        Fx(Eff::Immediate(s.clone(), s))
    }
}

impl<'f, S: Clone, V: Clone> Fx<'f, S, V> {
    pub fn immediate(s: S, value: V) -> Self {
        Fx(Eff::Immediate(s, value))
    }

    pub fn value(value: V) -> Self {
        Fx::pending(|s: S| Fx(Eff::Immediate(s, value)))
    }

    pub fn pending<F>(f: F) -> Self
    where
        F: FnOnce(S) -> Self + Clone + 'f,
    {
        Fx(Eff::Pending(Box::new(f)))
    }

    pub fn cmap<T, C, F>(self, c: C, f: F) -> Fx<'f, T, V>
    where
        T: Clone,
        C: FnOnce(T) -> S + Clone + 'f,
        F: FnOnce(S) -> T + Clone + 'f,
    {
        self.adapt(|t: T| c(t), |s, v| Fx(Eff::Immediate(f(s), v)))
    }

    pub fn adapt<T, U, C, F>(self, cmap: C, fmap: F) -> Fx<'f, T, U>
    where
        T: Clone,
        S: Clone,
        U: Clone,
        C: FnOnce(T) -> S + Clone + 'f,
        F: FnOnce(S, V) -> Fx<'f, T, U> + Clone + 'f,
    {
        Fx::pending(|t: T| match self.0 {
            Eff::Immediate(s, v) => fmap(s, v),
            Eff::Pending(f) => f(cmap.clone()(t)).adapt(cmap, fmap),
        })
    }
}
