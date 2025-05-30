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
                Eff::Pending(mut f) => e = f(Nil),
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
        F: FnMut(S) -> Self + Clone + 'f,
    {
        Fx(Eff::Pending(Box::new(f)))
    }

    pub fn stopped<F>(f: F) -> Self
    where
        F: FnMut() -> Self + Clone + 'f,
    {
        Fx(Eff::Stopped(Box::new(f)))
    }

    pub fn start<F>(self, mut r: F) -> Self
    where
        F: FnMut(Self) -> Self + Clone + 'f,
    {
        match self.0 {
            Eff::Stopped(mut f) => r(f()),
            Eff::Immediate(v) => Fx::immediate(v),
            Eff::Pending(mut f) => Fx::pending(move |s: S| f(s).start(r.clone())),
        }
    }

    pub fn adapt<T, U, C, F>(self, mut cmap: C, mut fmap: F) -> Fx<'f, T, U>
    where
        U: Clone,
        C: FnMut(T) -> S + Clone + 'f,
        F: FnMut(V) -> Fx<'f, T, U> + Clone + 'f,
    {
        match self.0 {
            Eff::Immediate(v) => fmap(v),
            Eff::Stopped(mut f) => Fx::stopped(move || f().adapt(cmap.clone(), fmap.clone())),
            Eff::Pending(mut f) => {
                Fx::pending(move |t: T| f(cmap(t)).adapt(cmap.clone(), fmap.clone()))
            }
        }
    }
}

impl<'f, S: 'f, V: Clone> Clone for Fx<'f, S, V> {
    fn clone(&self) -> Self {
        Fx(self.0.clone())
    }
}
