use super::And;

#[derive(Copy, Clone)]
pub struct Nil;

pub struct Fx<'f, S, V> (FxPriv<'f, S, V>);

enum FxPriv<'f, S: 'f, V: 'f>
{
    Immediate(V),
    Pending(Box<dyn Fn(S) -> Fx<'f, S, V> + 'f>),
    Stopped(Box<dyn Fn() -> Fx<'f, S, V> + 'f>),
}

impl <'f, V> Fx<'f, Nil, V>
{
    pub fn eval(self) -> Option<V> {
        let mut e = self;
        loop { 
            match e.0 {
                FxPriv::Immediate(v) => return Some(v),
                FxPriv::Stopped(_) => return None,
                FxPriv::Pending(f) => e = f(Nil)
            }
        }
    }
}

impl <'f, S, V> Fx<'f, S, V>
{
    pub fn immediate(value: V) -> Self {
        Fx(FxPriv::Immediate(value))
    }

    pub fn pending<F: Fn(S) -> Self + 'f>(f: F) -> Self {
        Fx(FxPriv::Pending(Box::new(f)))
    }

    pub fn stopped<F: Fn() -> Self + 'f>(f: F) -> Self {
        Fx(FxPriv::Stopped(Box::new(f)))
    }

    pub fn start<F>(self, r: F) -> Self 
    where F: Fn(Self) -> Self + Copy + 'f {
        match self.0 {
            FxPriv::Stopped(f) => r(f()),
            FxPriv::Immediate(v) => Fx::immediate(v),
            FxPriv::Pending(f) => Fx::pending(move |s: S| f(s).start(r) )
        }
    }

    pub fn then<T, U, C, F>(self, cmap: C, fmap: F) -> Fx<'f, T, U>
    where
        C: Fn(T) -> S + Copy + 'f,
        F: Fn(V) -> Fx<'f, T, U> + Copy +'f,
    {
        match self.0 {
            FxPriv::Immediate(v) => fmap(v),
            FxPriv::Stopped(f) => Fx::stopped(move || { f().then(cmap, fmap) }),
            FxPriv::Pending(f) => Fx::pending(move |t: T| { f(cmap(t)).then(cmap, fmap) })
        }
    }
}

impl <'a, A: Copy, B: Copy, V> Fx<'a, And<A, B>, V> 
{
    fn rec_provide_left(self, ab: And<A, B>) -> Fx<'a, B, V> {
        match self.0 {
            FxPriv::Immediate(v) => Fx::immediate(v),
            FxPriv::Stopped(f) => Fx::stopped(move || f().rec_provide_left(ab)),
            FxPriv::Pending(f) => f(ab).rec_provide_left(ab),
        } 
    }

    pub fn provide_left(self, a: A) -> Fx<'a, B, V> {
        match self.0 {
            FxPriv::Immediate(v) => Fx::immediate(v),
            FxPriv::Stopped(f) => Fx::stopped(move || f().provide_left(a)),
            FxPriv::Pending(f) => Fx::pending(move |b: B| {
                let ab = And::new(a, b);
                f(ab).rec_provide_left(ab)
            }),
        }
    }
}
