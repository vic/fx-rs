use super::And;

#[derive(Copy, Clone)]
pub struct Nil;

pub struct Fx<'f, S, V>(Eff<'f, S, V>);

enum Eff<'f, S, V> {
    Immediate(V),
    Pending(Continue<'f, S, V>),
    Stopped(Resume<'f, S, V>),
}

struct Continue<'f, S: 'f, V: 'f>(Box<dyn Fn(S) -> Fx<'f, S, V> + 'f>);
struct Resume<'f, S, V>(Box<dyn Fn() -> Fx<'f, S, V> + 'f>);

impl<'f, V> Fx<'f, Nil, V> {
    pub fn eval(self) -> Option<V> {
        let mut e = self;
        loop {
            match e.0 {
                Eff::Immediate(v) => return Some(v),
                Eff::Stopped(_) => return None,
                Eff::Pending(f) => e = f.0(Nil),
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
        Fx(Eff::Pending(Continue(Box::new(f))))
    }

    pub fn stopped<F: Fn() -> Self + 'f>(f: F) -> Self {
        Fx(Eff::Stopped(Resume(Box::new(f))))
    }

    pub fn start<F>(self, r: F) -> Self
    where
        F: Fn(Self) -> Self + Copy + 'f,
    {
        match self.0 {
            Eff::Stopped(f) => r(f.0()),
            Eff::Immediate(v) => Fx::immediate(v),
            Eff::Pending(f) => Fx::pending(move |s: S| f.0(s).start(r)),
        }
    }

    pub fn then<T, U, C, F>(self, cmap: C, fmap: F) -> Fx<'f, T, U>
    where
        C: Fn(T) -> S + Copy + 'f,
        F: Fn(V) -> Fx<'f, T, U> + Copy + 'f,
    {
        match self.0 {
            Eff::Immediate(v) => fmap(v),
            Eff::Stopped(f) => Fx::stopped(move || f.0().then(cmap, fmap)),
            Eff::Pending(f) => Fx::pending(move |t: T| f.0(cmap(t)).then(cmap, fmap)),
        }
    }
}

impl<'a, A: Copy, B: Copy, V> Fx<'a, And<A, B>, V> {
    fn rec_provide_left(self, ab: And<A, B>) -> Fx<'a, B, V> {
        match self.0 {
            Eff::Immediate(v) => Fx::immediate(v),
            Eff::Stopped(f) => Fx::stopped(move || f.0().rec_provide_left(ab)),
            Eff::Pending(f) => f.0(ab).rec_provide_left(ab),
        }
    }

    pub fn provide_left(self, a: A) -> Fx<'a, B, V> {
        match self.0 {
            Eff::Immediate(v) => Fx::immediate(v),
            Eff::Stopped(f) => Fx::stopped(move || f.0().provide_left(a)),
            Eff::Pending(f) => Fx::pending(move |b: B| {
                let ab = And::new(a, b);
                f.0(ab).rec_provide_left(ab)
            }),
        }
    }
}
