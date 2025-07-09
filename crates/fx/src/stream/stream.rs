use crate::core::acc::Acc;
use crate::core::state::State;
use crate::kernel::fx::Fx;

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

    pub fn fold_stream_rec<A, F>(current: A, stream: StreamFx<'f, S, I>, f: F) -> Fx<'f, S, A>
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

    pub fn adapt<T, U, Getter, Setter, FMap>(
        self,
        getter: Getter,
        setter: Setter,
        fmap: FMap,
    ) -> Stream<'f, T, U>
    where
        T: Clone + 'f,
        U: Clone + 'f,
        Getter: FnOnce(T) -> S + Clone + 'f,
        Setter: FnOnce(T, S) -> T + Clone + 'f,
        FMap: FnOnce(I) -> U + Clone + 'f,
    {
        match self {
            Stream::Nil => Stream::Nil,
            Stream::Cons(head, fx) => {
                let getter0 = getter.clone();
                let setter0 = setter.clone();
                let head = fmap.clone()(head);
                let fx = fx.adapt(
                    |t: T| getter0(t),
                    |t, s, stream| Fx::immediate(setter0(t, s), stream.adapt(getter, setter, fmap)),
                );
                Stream::cons(head, fx)
            }
        }
    }
}

impl<'f, S: Clone, I: Clone> Acc<'f, S, I> for Stream<'f, S, I> {
    fn acc_fx(self, i: I) -> Fx<'f, S, Self> {
        Fx::value(Stream::append(self, i))
    }
}
