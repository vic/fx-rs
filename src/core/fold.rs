use crate::{Ability, Fx, Handler, State};

pub trait Fold<'f, I, S, O, Acc>
where
    I: Clone,
    S: Clone,
    O: Clone,
    Acc: Clone,
{
    fn fold_with<V: Clone>(self, acc: Acc) -> FoldHandler<'f, I, S, O, V, Acc>;

    fn fold_default<V: Clone>(self) -> FoldHandler<'f, I, S, O, V, Acc>
    where
        Self: Sized,
        Acc: Default,
    {
        self.fold_with(Default::default())
    }
}

pub type FoldAbility<'f, I, S, O, Acc> = Ability<'f, I, (Acc, S), O>;

pub type FoldHandler<'f, I, S, O, V, Acc> =
    Handler<'f, (FoldAbility<'f, I, S, O, Acc>, (Acc, S)), S, V, (Acc, V)>;

impl<'f, I, S, O> Fold<'f, I, S, O, Vec<O>> for FoldAbility<'f, I, S, O, Vec<O>>
where
    I: Clone,
    S: Clone,
    O: Clone,
{
    fn fold_with<V: Clone>(self, acc: Vec<O>) -> FoldHandler<'f, I, S, O, V, Vec<O>> {
        self.fold(acc, |mut vec, o| {
            Fx::value({
                vec.push(o);
                vec
            })
        })
    }
}

impl<'f, I, S, O> Fold<'f, I, S, O, Option<O>> for FoldAbility<'f, I, S, O, Option<O>>
where
    I: Clone,
    S: Clone,
    O: Clone,
{
    fn fold_with<V: Clone>(self, acc: Option<O>) -> FoldHandler<'f, I, S, O, V, Option<O>> {
        self.fold(acc, |_opt, o| Fx::value(Some(o)))
    }
}

impl<'f, I, S, O, Acc> FoldAbility<'f, I, S, O, Acc>
where
    O: Clone,
    S: Clone + 'f,
    I: Clone + 'f,
    Acc: Clone + 'f,
{
    pub fn fold<V, F>(self, acc: Acc, of: F) -> FoldHandler<'f, I, S, O, V, Acc>
    where
        V: Clone + 'f,
        F: (FnOnce(Acc, O) -> Fx<'f, S, Acc>) + Clone + 'f,
    {
        Handler::new(|e| {
            let ab = self.hmap(|f| {
                f.map_m(|o| {
                    let o1 = o.clone();
                    State::update(|(a, s): (Acc, S)| of(a, o1).map(|a| (a, s)))
                        .contra_map::<(Acc, S), _, _>(|(o, s)| ((o, s.clone()), s), |_, (os, _)| os)
                        .map(|_| o)
                })
            });

            e.map_m(|v| State::get().map(|(_ab, (a, _s)): (Self, (Acc, S))| (a, v)))
                .provide_left(ab)
                .provide_left(acc)
        })
    }
}
