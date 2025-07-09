use crate::{
    core::{handler::Handler, pair::Pair, state::State},
    kernel::fx::Fx,
};

pub struct Arrows;
impl Arrows {
    pub fn request<'f, I, O, A>(i: I) -> Fx<'f, A, O>
    where
        I: Clone + 'f,
        O: Clone + 'f,
        A: Arrow<'f, I, O> + Clone + 'f,
    {
        State::get().map(|f: A| f.apply(i))
    }

    pub fn new<'f, I, O, F>(f: F) -> impl Arrow<'f, I, O> + Clone + 'f
    where
        I: Clone + 'f,
        O: Clone + 'f,
        F: FnOnce(I) -> O + Clone + 'f,
    {
        f
    }
}

impl<'f, I, O, F> Arrow<'f, I, O> for F
where
    O: Clone + 'f,
    I: Clone + 'f,
    F: FnOnce(I) -> O + Clone + 'f,
{
    fn apply(self, i: I) -> O {
        self(i)
    }
}

pub trait Arrow<'f, I, O>
where
    O: Clone + 'f,
    I: Clone + 'f,
    Self: Clone + 'f,
{
    fn apply(self, i: I) -> O;

    fn handler<B, V, P, F>(self) -> impl Handler<'f, P, B, V, V>
    where
        F: FnOnce(I) -> O + Clone + 'f,
        B: Clone + 'f,
        V: Clone + 'f,
        P: Pair<Self, B> + 'f,
    {
        |e: Fx<'f, P, V>| e.provide_left(self)
    }

    fn adapt<T, U, H, F>(self, cmap: H, fmap: F) -> impl Arrow<'f, T, U>
    where
        T: Clone + 'f,
        U: Clone + 'f,
        H: FnOnce(T) -> I + Clone + 'f,
        F: FnOnce(O) -> U + Clone + 'f,
    {
        |t: T| fmap(self.apply(cmap(t)))
    }
}
