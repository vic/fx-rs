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
pub enum Stream<'f, S: Clone, I: Clone> {
    Nil,
    Cons(I, Box<StreamFx<'f, S, I>>),
}

impl<'f, S: Clone, I: Clone> Default for Stream<'f, S, I> {
    fn default() -> Self {
        Stream::Nil
    }
}

pub type StreamFx<'f, S, I> = Fx<'f, S, Stream<'f, S, I>>;

impl<'f, S: Clone, I: Clone> Stream<'f, S, I> {
    pub fn empty() -> Self {
        Self::Nil
    }

    pub fn cons(head: I, tail: StreamFx<'f, S, I>) -> Self {
        Self::Cons(head, Box::new(tail))
    }

    pub fn single(i: I) -> Self {
        Self::cons(i, Fx::value(Self::empty()))
    }

    pub fn concat(self, right: Stream<'f, S, I>) -> Self {
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
        State::get().flat_map(|initial: A| Self::fold_stream_rec(initial, Fx::value(self), f))
    }

    pub fn fold_stream<A, F>(f: F) -> Fx<'f, (Self, (A, S)), A>
    where
        A: Clone + 'f,
        F: FnOnce(A, I) -> Fx<'f, S, Item<A>> + Clone + 'f,
    {
        State::get()
            .flat_map(|(stream, initial): (Self, A)| {
                Self::fold_stream_rec(initial, Fx::value(stream), f)
            })
            .contra_map(|(s, (a, r))| ((s, a), r), |_, ((s, a), r)| (s, (a, r)))
    }

    fn fold_stream_rec<A, F>(current: A, stream: StreamFx<'f, S, I>, f: F) -> Fx<'f, S, A>
    where
        A: Clone + 'f,
        F: FnOnce(A, I) -> Fx<'f, S, Item<A>> + Clone + 'f,
    {
        stream.map_m(|step| {
            match step {
                Stream::Nil => Fx::value(current),
                Stream::Cons(head, tail) => {
                    let f0 = f.clone();
                    let acc = f(current, head);
                    acc.map_m(|acc| match acc {
                        Item::Done(a) => Fx::value(a), // TODO: stop stream producer
                        Item::Next(a) => Self::fold_stream_rec(a, *tail, f0),
                    })
                }
            }
        })
    }
}

impl<'f, S, I, Item> Fold<'f, I, S, Item, Stream<'f, S, Item>>
    for FoldAbility<'f, I, S, Item, Stream<'f, S, Item>>
where
    I: Clone,
    S: Clone,
    Item: Clone,
{
    fn fold_with<V: Clone>(
        self,
        acc: Stream<'f, S, Item>,
    ) -> FoldHandler<'f, I, S, Item, V, Stream<'f, S, Item>> {
        self.fold(acc, |acc, item| Fx::value(Stream::append(acc, item)))
    }
}
