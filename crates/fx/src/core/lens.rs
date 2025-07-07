use dyn_clone::{DynClone, clone_trait_object};

use crate::{Fx, Handler, Pair, Has, Put};

#[derive(Clone)]
pub struct Lens<'f, Outer: Clone, Inner: Clone>(Get<'f, Outer, Inner>, Set<'f, Outer, Inner>);

impl<'f, Outer: Clone, Inner: Clone> Lens<'f, Outer, Inner> {
    pub fn new<G, S>(getter: G, setter: S) -> Self
    where
        G: FnOnce(Outer) -> Inner + Clone + 'f,
        S: FnOnce(Outer, Inner) -> Outer + Clone + 'f,
    {
        Self(Box::new(getter), Box::new(setter))
    }

    pub fn zoom_out<V: Clone>(self) -> Handler<'f, Inner, Outer, V, V> {
        Handler::new(|e| e.contra_map(self.0, self.1))
    }

    pub fn zoom_in<V: Clone, U: Clone, F>(self, inner: F) -> Handler<'f, Outer, Outer, V, U>
    where
        Inner: 'f,
        U: Clone + 'f,
        F: FnOnce(V) -> Fx<'f, Inner, U> + Clone + 'f,
    {
        Handler::new(|e| e.map_m(|v| inner(v).via(self.zoom_out())))
    }

    pub fn prepend<LeftOuter: Clone>(
        self,
        left: Lens<'f, LeftOuter, Outer>,
    ) -> Lens<'f, LeftOuter, Inner>
    where
        Inner: 'f,
        LeftOuter: 'f,
        Outer: 'f,
    {
        let reader = (left.0).clone();
        Lens::<LeftOuter, Inner>::new(
            |left_outer| self.0(reader(left_outer)),
            |left_outer, inner| left.1(left_outer.clone(), self.1(left.0(left_outer), inner)),
        )
    }

    pub fn append<RightInner: Clone>(
        self,
        right: Lens<'f, Inner, RightInner>,
    ) -> Lens<'f, Outer, RightInner>
    where
        Inner: 'f,
        RightInner: 'f,
        Outer: 'f,
    {
        let reader = (self.0).clone();
        Lens::<Outer, RightInner>::new(
            |outer| right.0(reader(outer)),
            |outer, right_inner| self.1(outer.clone(), right.1(self.0(outer), right_inner)),
        )
    }

    pub fn from_has_put() -> Self
    where
        Outer: Has<Inner> + Put<Inner> + Clone,
        Inner: Clone,
    {
        Self(
            Box::new(|outer: Outer| outer.get().clone()),
            Box::new(|outer: Outer, inner: Inner| outer.put(inner)),
        )
    }

    pub fn get(&self, outer: Outer) -> Inner {
        (self.0.clone())(outer)
    }
    pub fn set(&self, outer: Outer, inner: Inner) -> Outer {
        (self.1.clone())(outer, inner)
    }
}

impl<'f, A: Clone, P: Clone> Lens<'f, P, A> {
    pub fn left<B>() -> Self
    where
        B: Clone,
        P: Pair<A, B>,
    {
        Self::new(|p| p.fst(), |p, a| P::from((a, p.snd())))
    }
}

impl<'f, B: Clone, P: Clone> Lens<'f, P, B> {
    pub fn right<A>() -> Self
    where
        B: Clone,
        P: Pair<A, B>,
    {
        Self::new(|p| p.snd(), |p, b| P::from((p.fst(), b)))
    }
}

type Get<'f, Outer, Inner> = Box<dyn GetterFn<'f, Outer, Inner> + 'f>;

clone_trait_object!(<'f, Outer:Clone, Inner:Clone> GetterFn<'f, Outer, Inner>);
trait GetterFn<'f, Outer: Clone, Inner: Clone>
where
    Self: DynClone + FnOnce(Outer) -> Inner + 'f,
{
}

impl<'f, Outer: Clone, Inner: Clone, F> GetterFn<'f, Outer, Inner> for F where
    F: FnOnce(Outer) -> Inner + Clone + 'f
{
}

type Set<'f, Outer, Inner> = Box<dyn SetterFn<'f, Outer, Inner> + 'f>;

clone_trait_object!(<'f, Outer:Clone, Inner:Clone> SetterFn<'f, Outer, Inner>);
trait SetterFn<'f, Outer: Clone, Inner: Clone>
where
    Self: DynClone + FnOnce(Outer, Inner) -> Outer + 'f,
{
}

impl<'f, Outer: Clone, Inner: Clone, F> SetterFn<'f, Outer, Inner> for F where
    F: FnOnce(Outer, Inner) -> Outer + Clone + 'f
{
}

#[cfg(test)]
#[path = "lens_test.rs"]
mod lens_test;
