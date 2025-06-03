use super::eff::Eff;

#[derive(Copy, Clone)]
pub struct Nil;

pub struct Fx<'f, S, V: Clone>(pub(super) Eff<'f, S, V>);

impl<V: Clone> Fx<'_, Nil, V> {
    pub fn eval(self) -> Option<V> {
        let mut e = self;
        loop {
            match e.0 {
                Eff::Immediate(v) => return Some(v),
                Eff::Stopped(_) => return None,
                Eff::Pending(f) => e = f(Nil),
                Eff::Provided(s, f) => e = f(s),
            }
        }
    }
}

impl<'f, S, V: Clone> Fx<'f, S, V> {
    pub fn immediate(value: V) -> Self {
        Fx(Eff::Immediate(value))
    }

    pub fn pending<F>(f: F) -> Self
    where
        F: FnOnce(S) -> Self + Clone + 'f,
    {
        Fx(Eff::Pending(Box::new(f)))
    }

    pub fn stopped<F>(f: F) -> Self
    where
        F: FnOnce() -> Self + Clone + 'f,
    {
        Fx(Eff::Stopped(Box::new(f)))
    }

    pub(crate) fn provide<T: Clone>(self, s: S) -> Fx<'f, T, V>
    where
        S: Clone,
    {
        Fx(Eff::Provided(s.clone(), Box::new(move |_| self))).adapt(move |_| s, Fx::immediate)
    }

    pub fn start<F>(self, r: F) -> Self
    where
        F: FnOnce(Self) -> Self + Clone + 'f,
        S: Clone,
    {
        match self.0 {
            Eff::Stopped(f) => r(f()),
            Eff::Immediate(v) => Fx::immediate(v),
            Eff::Pending(f) => Fx::pending(move |s: S| f(s).start(r.clone())),
            Eff::Provided(s, f) => Fx::pending(move |s: S| f(s).start(r.clone())).provide(s),
        }
    }

    pub fn adapt<T, U, C, F>(self, cmap: C, fmap: F) -> Fx<'f, T, U>
    where
        U: Clone,
        S: Clone,
        C: FnOnce(T) -> S + Clone + 'f,
        F: FnOnce(V) -> Fx<'f, T, U> + Clone + 'f,
    {
        match self.0 {
            Eff::Immediate(v) => fmap(v),
            Eff::Stopped(f) => Fx::stopped(move || f().adapt(cmap, fmap)),
            Eff::Pending(f) => Fx::pending(move |t: T| f(cmap.clone()(t)).adapt(cmap, fmap)),
            Eff::Provided(s, f) => Self::propagate_provided(s, f, fmap),
        }
    }

    fn propagate_provided<T, U, C, F>(s: S, f: C, fmap: F) -> Fx<'f, T, U>
    where
        U: Clone,
        S: Clone,
        C: FnOnce(S) -> Fx<'f, S, V> + Clone + 'f,
        F: FnOnce(V) -> Fx<'f, T, U> + Clone + 'f,
    {
        f(s.clone()).adapt(move |_: T| s, fmap)
    }
}

impl<'f, S: Clone + 'f, V: Clone> Clone for Fx<'f, S, V> {
    fn clone(&self) -> Self {
        Fx(self.0.clone())
    }
}
