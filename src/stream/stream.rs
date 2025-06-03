use crate::{Ability, Fx};

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
        Fx::immediate(Self::Nil)
    }

    pub fn cons(head: I, tail: StreamFx<'f, I, S>) -> StreamFx<'f, I, S> {
        Fx::immediate(Self::Cons(head, Box::new(tail)))
    }

    pub fn once(i: I) -> StreamFx<'f, I, S> {
        Fx::immediate(Self::Cons(i, Box::new(Self::empty())))
    }

    pub fn emit(i: Item<I>) -> Fx<'f, (EmitCap<'f, Item<I>, S>, S), ()> {
        EmitCap::request(i)
    }
}

pub type EmitCap<'f, I, S> = Ability<'f, I, S, ()>;

impl<'f, I: Clone, S: Clone> StreamFx<'f, I, S> {
    pub fn fold_stream<A, F>(f: F) -> Fx<'f, (Self, (A, S)), A>
    where
        A: Clone + 'f,
        F: FnOnce(A, I) -> Fx<'f, S, Item<A>> + Clone + 'f,
    {
        Fx::ctx()
            .flat_map(move |(stream, initial): (Self, A)| {
                Self::fold_stream_rec(initial, stream, f.clone())
            })
            .contra_map(|(s, (a, r))| ((s, a), r))
    }

    pub fn fold<A, F>(self, f: F) -> Fx<'f, (A, S), A>
    where
        A: Clone + 'f,
        F: FnOnce(A, I) -> Fx<'f, S, Item<A>> + Clone + 'f,
    {
        Fx::ctx()
            .flat_map(move |initial: A| Self::fold_stream_rec(initial, self.clone(), f.clone()))
    }

    fn fold_stream_rec<A, F>(current: A, stream: Self, f: F) -> Fx<'f, S, A>
    where
        A: Clone + 'f,
        F: FnOnce(A, I) -> Fx<'f, S, Item<A>> + Clone + 'f,
    {
        stream.map_m(move |step| {
            match step {
                Stream::Nil => Fx::immediate(current.clone()),
                Stream::Cons(head, tail) => {
                    let f0 = f.clone();
                    let acc = f(current.clone(), head);
                    acc.map_m(move |acc| match acc {
                        Item::Done(a) => Fx::immediate(a), // TODO: stop stream producer
                        Item::Next(a) => Self::fold_stream_rec(a, (&*tail).clone(), f0),
                    })
                }
            }
        })
    }
}
