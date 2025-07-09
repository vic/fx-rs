use crate::{
    core::{ability::AbilityExt, handler::Handler, state::State},
    kernel::{ability::Ability, fx::Fx},
};

/// An effectful accumulator of I
pub trait Acc<'f, S: Clone, I>: Clone {
    fn acc_fx(self, i: I) -> Fx<'f, S, Self>;
}

impl<'f, S: Clone, I: Clone> Acc<'f, S, I> for Option<I> {
    fn acc_fx(self, i: I) -> Fx<'f, S, Self> {
        Fx::value(Some(i))
    }
}

impl<'f, S: Clone, I: Clone> Acc<'f, S, I> for Vec<I> {
    fn acc_fx(mut self, i: I) -> Fx<'f, S, Self> {
        Fx::pending(|_| {
            self.push(i);
            Fx::value(self)
        })
    }
}

pub trait AccAbilityExt<'f, I, S, O, A>:
    Ability<'f, I, (A, S), O> + AbilityExt<'f, I, (A, S), O> + Clone + 'f
where
    I: Clone + 'f,
    S: Clone + 'f,
    O: Clone + 'f,
    A: Clone + 'f,
{
    fn acc_outcome_with<V: Clone + 'f, F>(
        self,
        acc: A,
        f: F,
    ) -> impl Handler<'f, (Box<dyn Ability<'f, I, (A, S), O> + 'f>, (A, S)), S, V, (A, V)> + Clone + 'f
    where
        F: FnOnce(A, O) -> Fx<'f, S, A> + Clone + 'f,
        Self: Sized + 'f,
    {
        acc_handler(Box::new(self), acc, f)
    }

    fn acc_outcome<V: Clone + 'f>(
        self,
        acc: A,
    ) -> impl Handler<'f, (Box<dyn Ability<'f, I, (A, S), O> + 'f>, (A, S)), S, V, (A, V)> + Clone + 'f
    where
        A: Acc<'f, S, O>,
        Self: Sized + 'f,
    {
        acc_handler(Box::new(self), acc, |acc, i| acc.acc_fx(i))
    }

    fn acc_outcome_default<V: Clone + 'f>(
        self,
    ) -> impl Handler<'f, (Box<dyn Ability<'f, I, (A, S), O> + 'f>, (A, S)), S, V, (A, V)> + Clone + 'f
    where
        A: Acc<'f, S, O> + Default,
        Self: Sized + 'f,
    {
        self.acc_outcome(Default::default())
    }
}

impl<'f, I, S, O, A, T> AccAbilityExt<'f, I, S, O, A> for T
where
    I: Clone + 'f,
    S: Clone + 'f,
    O: Clone + 'f,
    A: Clone + 'f,
    T: Ability<'f, I, (A, S), O> + AbilityExt<'f, I, (A, S), O> + Clone + 'f,
{
}

fn acc_handler<'f, A, V, I, S, O, F>(
    ab: Box<dyn Ability<'f, I, (A, S), O> + 'f>,
    a: A,
    op: F,
) -> impl Handler<'f, (Box<dyn Ability<'f, I, (A, S), O> + 'f>, (A, S)), S, V, (A, V)> + Clone + 'f
where
    A: Clone + 'f,
    I: Clone + 'f,
    S: Clone + 'f,
    O: Clone + 'f,
    V: Clone + 'f,
    F: FnOnce(A, O) -> Fx<'f, S, A> + Clone + 'f,
{
    move |e: Fx<'f, (Box<dyn Ability<'f, I, (A, S), O> + 'f>, (A, S)), V>| {
        let ab_boxed: Box<dyn Ability<'f, I, (A, S), O> + 'f> = Box::new(ab.clone().hmap(|f| {
            f.map_m(|o| {
                let o1 = o.clone();
                State::update(|(a, s): (A, S)| op(a, o1).map(|a| (a, s)))
                    .map(|_| o)
                    .contra_map(|(a, s): (A, S)| ((a, s.clone()), s), |_, (st, _)| st)
            })
        }));

        e.provide_left(ab_boxed)
            .map_m(|v| State::get().map(|(a, _)| (a, v)))
            .provide_left(a)
    }
}
