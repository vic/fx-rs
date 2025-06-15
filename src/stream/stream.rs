use crate::{
    Fold, Fx, State,
    core::{FoldAbility, FoldHandler},
};

#[derive(Clone)]
pub enum Item<T> {
    Next(T),
    Done(T),
}

#[derive(Clone)]
pub enum Stream<'f, I: Clone, S: Clone> {
    Nil,
    Cons(I, Box<StreamFx<'f, I, S>>),
}

impl<'f, I: Clone, S: Clone> Default for Stream<'f, I, S> {
    fn default() -> Self {
        Stream::Nil
    }
}

pub type StreamFx<'f, I, S> = Fx<'f, S, Stream<'f, I, S>>;

impl<'f, I: Clone, S: Clone> Stream<'f, I, S> {
    pub fn empty() -> Self {
        Self::Nil
    }

    pub fn cons(head: I, tail: StreamFx<'f, I, S>) -> Self {
        Self::Cons(head, Box::new(tail))
    }

    pub fn single(i: I) -> Self {
        Self::cons(i, Fx::value(Self::empty()))
    }

    pub fn concat(self, right: Stream<'f, I, S>) -> Self {
        match self {
            Stream::Nil => right,
            Stream::Cons(head, fx) => Stream::cons(head, fx.map(|s| s.concat(right))),
        }
    }

    pub fn append(self, item: I) -> Self {
        self.concat(Self::single(item))
    }

    pub fn fold<A, F>(self, f: F) -> Fx<'f, (A, S), A>
    where
        A: Clone + 'f,
        F: FnOnce(A, I) -> Fx<'f, S, Item<A>> + Clone + 'f,
    {
        State::get()
            .flat_map(move |initial: A| Self::fold_stream_rec(initial, Fx::value(self), f.clone()))
    }

    pub fn fold_stream<A, F>(f: F) -> Fx<'f, (Self, (A, S)), A>
    where
        A: Clone + 'f,
        F: FnOnce(A, I) -> Fx<'f, S, Item<A>> + Clone + 'f,
    {
        State::get()
            .flat_map(move |(stream, initial): (Self, A)| {
                Self::fold_stream_rec(initial, Fx::value(stream), f.clone())
            })
            .contra_map(|(s, (a, r))| ((s, a), r), |_, ((s, a), r)| (s, (a, r)))
    }

    fn fold_stream_rec<A, F>(current: A, stream: StreamFx<'f, I, S>, f: F) -> Fx<'f, S, A>
    where
        A: Clone + 'f,
        F: FnOnce(A, I) -> Fx<'f, S, Item<A>> + Clone + 'f,
    {
        stream.map_m(move |step| {
            match step {
                Stream::Nil => Fx::value(current.clone()),
                Stream::Cons(head, tail) => {
                    let f0 = f.clone();
                    let acc = f(current.clone(), head);
                    acc.map_m(move |acc| match acc {
                        Item::Done(a) => Fx::value(a), // TODO: stop stream producer
                        Item::Next(a) => Self::fold_stream_rec(a, (&*tail).clone(), f0),
                    })
                }
            }
        })
    }
}

impl<'f, I, S, Item> Fold<'f, I, S, Item, Stream<'f, Item, S>>
    for FoldAbility<'f, I, S, Item, Stream<'f, Item, S>>
where
    I: Clone,
    S: Clone,
    Item: Clone,
{
    fn fold_with<V: Clone>(
        self,
        acc: Stream<'f, Item, S>,
    ) -> FoldHandler<'f, I, S, Item, V, Stream<'f, Item, S>> {
        self.fold(acc, |acc, item| Fx::value(Stream::append(acc, item)))
    }
}
