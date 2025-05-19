use super::And;

#[derive(Copy, Clone)]
pub struct Nil;

pub struct Fx<'f, S, V>(Eff<'f, S, V>);

enum Eff<'f, S: 'f, V: 'f> {
    Immediate(V),
    Pending(Continue<'f, S, V>),
    Stopped(Start<'f, S, V>),
}

type Continue<'f, S, V> = Box<dyn Fn(S) -> Fx<'f, S, V> + 'f>;
type Start<'f, S, V> = Box<dyn Fn() -> Fx<'f, S, V> + 'f>;

impl<V> Fx<'_, Nil, V> {
    pub fn eval(self) -> Option<V> {
        let mut e = self;
        loop {
            match e.0 {
                Eff::Immediate(v) => return Some(v),
                Eff::Stopped(_) => return None,
                Eff::Pending(f) => e = f(Nil),
            }
        }
    }
}

impl<'f, S, V> Fx<'f, S, V> {
    pub fn immediate(value: V) -> Self {
        Fx(Eff::Immediate(value))
    }

    pub fn pending<F>(f: F) -> Self
    where
        F: Fn(S) -> Self + 'f,
    {
        Fx(Eff::Pending(Box::new(f)))
    }

    pub fn stopped<F: Fn() -> Self + 'f>(f: F) -> Self {
        Fx(Eff::Stopped(Box::new(f)))
    }

    pub fn start<F>(self, r: F) -> Self
    where
        F: Fn(Self) -> Self + Copy + 'f,
    {
        match self.0 {
            Eff::Stopped(f) => r(f()),
            Eff::Immediate(v) => Fx::immediate(v),
            Eff::Pending(f) => Fx::pending(move |s: S| f(s).start(r)),
        }
    }

    pub fn then<T, U, C, F>(self, cmap: C, fmap: F) -> Fx<'f, T, U>
    where
        C: Fn(T) -> S + Copy + 'f,
        F: Fn(V) -> Fx<'f, T, U> + Copy + 'f,
    {
        match self.0 {
            Eff::Immediate(v) => fmap(v),
            Eff::Stopped(f) => Fx::stopped(move || f().then(cmap, fmap)),
            Eff::Pending(f) => Fx::pending(move |t: T| f(cmap(t)).then(cmap, fmap)),
        }
    }
}

impl<'a, A, B, V> Fx<'a, And<A, B>, V> {
    fn rec_provide_left(self, ab: And<A, B>) -> Fx<'a, B, V>
    where
        A: Clone,
        B: Clone,
    {
        match self.0 {
            Eff::Immediate(v) => Fx::immediate(v),
            Eff::Stopped(f) => Fx::stopped(move || f().rec_provide_left(ab.clone())),
            Eff::Pending(f) => f(ab.clone()).rec_provide_left(ab),
        }
    }

    pub fn provide_left(self, a: A) -> Fx<'a, B, V>
    where
        A: Clone,
        B: Clone,
    {
        match self.0 {
            Eff::Immediate(v) => Fx::immediate(v),
            Eff::Stopped(f) => Fx::stopped(move || f().provide_left(a.clone())),
            Eff::Pending(f) => Fx::pending(move |b: B| {
                let ab = And::new(a.clone(), b);
                f(ab.clone()).rec_provide_left(ab)
            }),
        }
    }
}
