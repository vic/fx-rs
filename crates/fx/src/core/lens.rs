use dyn_clone::{DynClone, clone_trait_object};

use crate::{Fx, Handler, Has, Pair, Put};

#[derive(Clone)]
pub struct Lens<'f, Outer: Clone, Inner: Clone>(Get<'f, Outer, Inner>, Set<'f, Outer, Inner>);

impl<'f, Outer: Clone + 'f, Inner: Clone + 'f> Lens<'f, Outer, Inner> {
    pub fn new() -> Self
    where
        Inner: Clone,
        Outer: Has<Inner> + Put<Inner> + Clone,
    {
        Self(
            Box::new(|outer: Outer| outer.get().clone()),
            Box::new(|outer: Outer, inner: Inner| outer.put(inner)),
        )
    }

    pub fn zoom_out<V: Clone + 'f>(&self) -> impl Handler<'f, Inner, Outer, V, V> {
        let get = self.0.clone();
        let set = self.1.clone();
        Box::new(|e: Fx<'f, Inner, V>| e.contra_map(get, set))
    }

    pub fn zoom_in<V: Clone + 'f, U: Clone + 'f, F>(
        &self,
        inner: F,
    ) -> impl Handler<'f, Outer, Outer, V, U>
    where
        Inner: 'f,
        F: FnOnce(V) -> Fx<'f, Inner, U> + Clone + 'f,
    {
        let get = self.0.clone();
        let set = self.1.clone();
        let inner = inner.clone();
        Box::new(move |e: Fx<'f, Outer, V>| {
            e.map_m(move |v| inner.clone()(v).via(Lens(get.clone(), set.clone()).zoom_out()))
        })
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
        let reader = left.0.clone();
        Lens(
            Box::new(move |left_outer| self.0.clone()(reader(left_outer))),
            Box::new(move |left_outer, inner| {
                left.1.clone()(
                    left_outer.clone(),
                    self.1.clone()(left.0.clone()(left_outer), inner),
                )
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
        let reader = self.0.clone();
        Lens(
            Box::new(move |outer| right.0.clone()(reader(outer))),
            Box::new(move |outer, right_inner| {
                self.1.clone()(
                    outer.clone(),
                    right.1.clone()(self.0.clone()(outer), right_inner),
                )
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

#[cfg(test)]
mod from_impl_test {
    use super::*;
    #[derive(Clone, Debug, PartialEq)]
    struct Ctx {
        a: u32,
        b: &'static str,
    }
    impl Has<u32> for Ctx {
        fn get(&self) -> &u32 {
            &self.a
        }
    }
    impl Put<u32> for Ctx {
        fn put(mut self, value: u32) -> Self {
            self.a = value;
            self
        }
    }
    #[test]
    fn from_has_put_lens() {
        let lens: Lens<'_, Ctx, u32> = Lens::new();
        let ctx = Ctx { a: 1, b: "hi" };
        assert_eq!(lens.get(ctx.clone()), 1);
        let updated = lens.set(ctx.clone(), 42);
        assert_eq!(updated, Ctx { a: 42, b: "hi" });
    }
}
