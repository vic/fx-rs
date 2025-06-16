use crate::{Ability, Fx, Handler, State};

/// An ability that has an accumulator as part of its environment.
pub type AccAbility<'f, I, S, O, A> = Ability<'f, I, (A, S), O>;

/// Folds an ability outcome O into an accumulator A.
pub type AccHandler<'f, I, S, O, A, V> =
    Handler<'f, (AccAbility<'f, I, S, O, A>, (A, S)), S, V, (A, V)>;

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

impl<'f, I, S, O, A> AccAbility<'f, I, S, O, A>
where
    I: Clone,
    S: Clone,
    O: Clone,
    A: Clone,
{
    pub fn acc_outcome_with<V: Clone, F>(self, acc: A, f: F) -> AccHandler<'f, I, S, O, A, V>
    where
        F: FnOnce(A, O) -> Fx<'f, S, A> + Clone + 'f,
    {
        acc_handler(self, acc, f)
    }

    pub fn acc_outcome<V: Clone>(self, acc: A) -> AccHandler<'f, I, S, O, A, V>
    where
        A: Acc<'f, S, O>,
    {
        acc_handler(self, acc, |acc, i| acc.acc_fx(i))
    }

    pub fn acc_outcome_default<V: Clone>(self) -> AccHandler<'f, I, S, O, A, V>
    where
        A: Acc<'f, S, O> + Default,
    {
        self.acc_outcome(Default::default())
    }
}

fn acc_handler<'f, A, V, I, S, O, F>(
    ab: AccAbility<'f, I, S, O, A>,
    a: A,
    op: F,
) -> AccHandler<'f, I, S, O, A, V>
where
    A: Clone,
    I: Clone,
    S: Clone,
    O: Clone,
    V: Clone,
    F: FnOnce(A, O) -> Fx<'f, S, A> + Clone + 'f,
{
    Handler::new(|e| {
        let ab = ab.hmap(|f| {
            f.map_m(|o| {
                let o1 = o.clone();
                State::update(|(a, s): (A, S)| op(a, o1).map(|a| (a, s)))
                    .map(|_| o)
                    .contra_map(|(a, s): (A, S)| ((a, s.clone()), s), |_, (st, _)| st)
            })
        });

        e.provide_left(ab)
            .map_m(|v| State::get().map(|(a, _)| (a, v)))
            .provide_left(a)
    })
}
