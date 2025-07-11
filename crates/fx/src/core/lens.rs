use dyn_clone::{DynClone, clone_trait_object};

use crate::{
    core::{
        handler::Handler,
        has_put::{HasPut, Put},
        pair::Pair,
    },
    kernel::fx::Fx,
};

#[derive(Clone)]
pub struct Lens<'f, Outer: Clone, Inner: Clone>(Get<'f, Outer, Inner>, Set<'f, Outer, Inner>);

impl<'f, Outer, Inner> Lens<'f, Outer, Inner>
where
    Inner: Clone + 'f,
    Outer: Clone + 'f,
{
    pub fn new() -> Self
    where
        Inner: Clone,
        Outer: HasPut<Inner, Outer> + Put<Inner, Outer> + Clone,
    {
        Self(
            Box::new(|outer: Outer| outer.get().clone()),
            Box::new(|outer: Outer, inner: Inner| outer.put(inner)),
        )
    }

    pub fn zoom_out<V>(self) -> impl Handler<'f, Inner, Outer, V, V>
    where
        V: Clone + 'f,
    {
        |e: Fx<'f, Inner, V>| e.contra_map(self.0, self.1)
    }

    pub fn zoom_in<V, U, F>(self, inner: F) -> impl Handler<'f, Outer, Outer, V, U>
    where
        Inner: 'f,
        V: Clone + 'f,
        U: Clone + 'f,
        F: FnOnce(V) -> Fx<'f, Inner, U> + Clone + 'f,
    {
        |e: Fx<'f, Outer, V>| e.map_m(|v| inner(v).via(self.zoom_out()))
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
        Lens(
            Box::new(|left_outer| self.0(reader(left_outer))),
            Box::new(|left_outer, inner| {
                left.1(left_outer.clone(), self.1(left.0(left_outer), inner))
            }),
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
        Lens(
            Box::new(|outer| right.0(reader(outer))),
            Box::new(|outer, right_inner| {
                self.1(outer.clone(), right.1(self.0(outer), right_inner))
            }),
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
        Lens(
            Box::new(|p| p.fst()),
            Box::new(|p, a| P::from((a, p.snd()))),
        )
    }
}

impl<'f, B: Clone, P: Clone> Lens<'f, P, B> {
    pub fn right<A>() -> Self
    where
        B: Clone,
        P: Pair<A, B>,
    {
        Lens(
            Box::new(|p| p.snd()),
            Box::new(|p, b| P::from((p.fst(), b))),
        )
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
