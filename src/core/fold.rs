use crate::{Ability, Fx, Handler, State};

pub type FoldAbility<'f, I, S, O, Acc> = Ability<'f, I, (Acc, S), O>;

pub type FoldHandler<'f, I, S, O, V, Acc> =
    Handler<'f, (FoldAbility<'f, I, S, O, Acc>, (Acc, S)), S, V, (Acc, V)>;

impl<'f, I, S, O> FoldAbility<'f, I, S, O, Vec<O>>
where
    O: Clone,
    S: Clone + 'f,
    I: Clone + 'f,
{
    pub fn fold_to_vec<V>(self) -> FoldHandler<'f, I, S, O, V, Vec<O>>
    where
        V: Clone,
    {
        self.fold(Vec::new(), |mut vec, o| {
            Fx::value({
                vec.push(o);
                vec
            })
        })
    }
}

impl<'f, I, S, O> FoldAbility<'f, I, S, O, Option<O>>
where
    O: Clone,
    S: Clone + 'f,
    I: Clone + 'f,
{
    pub fn fold_to_option<V>(self) -> FoldHandler<'f, I, S, O, V, Option<O>>
    where
        V: Clone,
    {
        self.fold(None, |_opt, o| Fx::value(Some(o)))
    }

    pub fn fold_to_some<V>(self) -> Handler<'f, (Self, (Option<O>, S)), S, V, (O, V)>
    where
        V: Clone,
    {
        self.fold_to_option()
            .map(|f: Fx<'f, S, (Option<O>, V)>| f.map(|(o, v)| (o.unwrap(), v)))
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
