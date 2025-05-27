use crate::Fx;

#[derive(Clone)]
pub enum Acc<T> {
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
}

impl<'f, I: Clone, S: Clone> StreamFx<'f, I, S> {
    pub fn fold<A, F>(f: F) -> Fx<'f, (Self, (A, S)), A>
    where
        A: Clone + 'f,
        F: Fn(A, I) -> Fx<'f, S, Acc<A>> + Clone + 'f,
    {
        Fx::ctx()
            .flat_map(move |(stream, initial): (Self, A)| {
                Self::fold_stream_rec(initial, stream, f.clone())
            })
            .contra_map(|(s, (a, r))| ((s, a), r))
    }

    pub fn fold_stream<A, F>(self, f: F) -> Fx<'f, (A, S), A>
    where
        A: Clone + 'f,
        F: Fn(A, I) -> Fx<'f, S, Acc<A>> + Clone + 'f,
    {
        Fx::ctx()
            .flat_map(move |initial: A| Self::fold_stream_rec(initial, self.clone(), f.clone()))
    }

    fn fold_stream_rec<A, F>(current: A, stream: Self, f: F) -> Fx<'f, S, A>
    where
        A: Clone + 'f,
        F: Fn(A, I) -> Fx<'f, S, Acc<A>> + Clone + 'f,
    {
        stream.map_m(move |step| {
            match step {
                Stream::Nil => Fx::immediate(current.clone()),
                Stream::Cons(head, tail) => {
                    let acc = f(current.clone(), head);
                    let f = f.clone();
                    acc.map_m(move |acc| match acc {
                        Acc::Done(a) => Fx::immediate(a), // TODO: stop stream producer
                        Acc::Next(a) => Self::fold_stream_rec(a, (&*tail).clone(), f.clone()),
                    })
                }
            }
        })
    }
}
