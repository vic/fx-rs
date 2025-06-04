use super::handler::Handler;
use crate::{Fx, kernel::Ctx};
use dyn_clone::{DynClone, clone_trait_object};

impl<'f, I, S, O> Ability<'f, I, S, O>
where
    O: Clone,
    S: Clone + 'f,
    I: Clone + 'f,
{
    pub fn request(i: I) -> Fx<'f, (Self, S), O>
    where
        I: Clone,
    {
        Ctx::get().flat_map(|f: Self| f.apply(i))
    }

    pub fn handler<F, B, V>(f: F) -> Handler<'f, (Self, B), B, V, V>
    where
        F: FnOnce(I) -> Fx<'f, S, O> + Clone + 'f,
        B: Clone,
        V: Clone,
    {
        Handler::new(|e: Fx<'f, (Self, B), V>| e.provide_left(Self::new(f)))
    }

    pub fn outcome_handler<F, B, V>(f: F) -> Handler<'f, (Self, B), B, V, (O, V)>
    where
        F: FnOnce(I) -> Fx<'f, S, O> + Clone + 'f,
        B: Clone,
        V: Clone,
    {
        Handler::new(|e: Fx<'f, (Self, B), V>| {
            let ab: Ability<'f, I, (Option<O>, S), O> = Ability::new(|i: I| {
                f(i).flat_map(|o: O| Ctx::set(Some(o)).map(|o| o.unwrap()))
                    .cmap(|(s, o)| (o, s), |(s, o)| (o, s))
            });

            let e = e.cmap(
                |(o, b): (Option<O>, B)| {
                    let ab = ab.cmap(|s: S| (o, s), |(o, s)| s);
                    (ab, b)
                },
                |(ab, b)| (None, b),
            );

            let e = e.map_m(|v: V| {
                Ctx::get().map(|(opt, _)| match opt {
                    None => unreachable!("Bug: ability outcome not propagated"),
                    Some(o) => (o, v),
                })
            });

            e.provide_left(None)
        })
    }

    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(I) -> Fx<'f, S, O> + Clone + 'f,
    {
        Self(Box::new(f))
    }

    pub fn apply(self, i: I) -> Fx<'f, S, O> {
        self.0(i)
    }

    pub fn clone_boxed(&self) -> Box<dyn FnOnce(I) -> Fx<'f, S, O> + 'f> {
        self.0.clone()
    }

    pub fn imap<Y, F>(self, imap: F) -> Ability<'f, Y, S, O>
    where
        Y: Clone + 'f,
        F: FnOnce(Y) -> I + Clone + 'f,
    {
        Ability::new(|y: Y| self.apply(imap(y)))
    }

    pub fn cmap<M, C, F>(self, cmap: C, fmap: F) -> Ability<'f, I, M, O>
    where
        M: Clone,
        C: FnOnce(M) -> S + Clone + 'f,
        F: FnOnce(S) -> M + Clone + 'f,
    {
        Ability::new(|i: I| self.apply(i).cmap(cmap, fmap))
    }

    pub fn map_m<U, F>(self, f: F) -> Ability<'f, I, S, U>
    where
        U: Clone,
        F: FnOnce(O) -> Fx<'f, S, U> + Clone + 'f,
    {
        Ability::new(|i: I| self.apply(i).map_m(f))
    }
}

#[derive(Clone)]
pub struct Ability<'f, I, S: Clone, O: Clone>(Box<dyn AbilityFn<'f, I, S, O> + 'f>);

clone_trait_object!(<'f, I, S: Clone, O: Clone> AbilityFn<'f, I, S, O>);

trait AbilityFn<'f, I, S, O>: DynClone + FnOnce(I) -> Fx<'f, S, O>
where
    O: Clone + 'f,
    S: Clone + 'f,
{
}

impl<'f, I, S, O, F> AbilityFn<'f, I, S, O> for F
where
    F: FnOnce(I) -> Fx<'f, S, O> + Clone,
    O: Clone + 'f,
    S: Clone + 'f,
{
}
