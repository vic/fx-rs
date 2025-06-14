use crate::{Fx, State};

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

pub type StreamFx<'f, I, S> = Fx<'f, S, Stream<'f, I, S>>;

impl<'f, I: Clone, S: Clone> Stream<'f, I, S> {
    pub fn empty() -> StreamFx<'f, I, S> {
        Fx::value(Self::Nil)
    }

    pub fn cons(head: I, tail: StreamFx<'f, I, S>) -> StreamFx<'f, I, S> {
        Fx::value(Self::Cons(head, Box::new(tail)))
    }

    pub fn single(i: I) -> StreamFx<'f, I, S> {
        Fx::value(Self::Cons(i, Box::new(Self::empty())))
    }

    pub fn concat(left: StreamFx<'f, I, S>, right: StreamFx<'f, I, S>) -> StreamFx<'f, I, S> {
        left.concat(right)
    }
}

impl<'f, I: Clone, S: Clone> StreamFx<'f, I, S> {
    pub fn concat(self, tail: Self) -> Self {
        self.map_m(|s| match s {
            Stream::Nil => tail,
            Stream::Cons(head, fx) => Stream::cons(head, Self::concat(*fx, tail)),
        })
    }

    pub fn fold_stream<A, F>(f: F) -> Fx<'f, (Self, (A, S)), A>
    where
        A: Clone + 'f,
        F: FnOnce(A, I) -> Fx<'f, S, Item<A>> + Clone + 'f,
    {
        State::get()
            .flat_map(move |(stream, initial): (Self, A)| {
                Self::fold_stream_rec(initial, stream, f.clone())
            })
            .contra_map(|(s, (a, r))| ((s, a), r), |_, ((s, a), r)| (s, (a, r)))
    }

    pub fn fold<A, F>(self, f: F) -> Fx<'f, (A, S), A>
    where
        A: Clone + 'f,
        F: FnOnce(A, I) -> Fx<'f, S, Item<A>> + Clone + 'f,
    {
        State::get()
            .flat_map(move |initial: A| Self::fold_stream_rec(initial, self.clone(), f.clone()))
    }

    fn fold_stream_rec<A, F>(current: A, stream: Self, f: F) -> Fx<'f, S, A>
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
