use crate::Fx;
use dyn_clone::{DynClone, clone_trait_object};

impl<'f, S: Clone, V: Clone> Fx<'f, S, V> {
    pub fn via<T, U, H>(self, h: H) -> Fx<'f, T, U>
    where
        T: Clone,
        U: Clone,
        H: Into<Handler<'f, S, T, V, U>>,
    {
        h.into().handle(self)
    }
}

impl<'f, A: Clone, B: Clone, U: Clone, V: Clone> Handler<'f, A, B, U, V> {
    pub fn on_left<S: Clone>(self) -> Handler<'f, (A, S), (B, S), U, V> {
        Handler::new(move |e| e.and_swap().via(self.clone().on_right()).and_swap())
    }

    pub fn on_right<S: Clone>(self) -> Handler<'f, (S, A), (S, B), U, V> {
        Handler::new(move |e| e.and_nest().via(self.clone().on_value()).and_flat())
    }

    pub fn on_value<S: Clone>(self) -> Handler<'f, S, S, Fx<'f, A, U>, Fx<'f, B, V>> {
        Handler::new(move |e: Fx<'f, S, Fx<'f, A, U>>| {
            e.flat_map(|i: Fx<'f, A, U>| Fx::func(move |h: Self| h.handle(i.clone())))
                .and_swap()
                .provide_left(self.clone())
        })
    }
}

impl<'f, A: Clone, B: Clone, U: Clone, V: Clone> Handler<'f, A, B, U, V> {
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(Fx<'f, A, U>) -> Fx<'f, B, V> + Clone + 'f,
    {
        Self(Box::new(f))
    }

    pub fn handle(self, e: Fx<'f, A, U>) -> Fx<'f, B, V> {
        self.0(e)
    }

    pub fn clone_boxed(&self) -> Box<dyn FnOnce(Fx<'f, A, U>) -> Fx<'f, B, V> + 'f> {
        self.0.clone()
    }

    pub fn contra_map<S, T, H>(self, h: H) -> Handler<'f, S, B, T, V>
    where
        H: FnOnce(Fx<'f, S, T>) -> Fx<'f, A, U> + Clone + 'f,
        S: Clone,
        T: Clone,
    {
        Handler::new(|e| self.handle(h(e)))
    }

    pub fn map<S, T, H>(self, h: H) -> Handler<'f, A, S, U, T>
    where
        H: FnOnce(Fx<'f, B, V>) -> Fx<'f, S, T> + Clone + 'f,
        S: Clone,
        T: Clone,
    {
        Handler::new(|e| h(self.handle(e)))
    }
}

#[derive(Clone)]
pub struct Handler<'f, A: Clone, B: Clone, U: Clone, V: Clone>(
    Box<dyn HandlerFn<'f, A, B, U, V> + 'f>,
);

clone_trait_object!(<'f, A: Clone, B: Clone, U:Clone, V:Clone> HandlerFn<'f, A, B, U, V>);
trait HandlerFn<'f, A, B, U, V>: DynClone + FnOnce(Fx<'f, A, U>) -> Fx<'f, B, V> + 'f
where
    V: Clone + 'f,
    U: Clone + 'f,
    A: Clone + 'f,
    B: Clone + 'f,
{
}
impl<'f, A, B, U, V, F> HandlerFn<'f, A, B, U, V> for F
where
    F: FnOnce(Fx<'f, A, U>) -> Fx<'f, B, V> + Clone + 'f,
    V: Clone + 'f,
    U: Clone + 'f,
    A: Clone + 'f,
    B: Clone + 'f,
{
}

impl<'f, A, B, U, V, F> From<F> for Handler<'f, A, B, U, V>
where
    F: FnOnce(Fx<'f, A, U>) -> Fx<'f, B, V> + Clone + 'f,
    V: Clone + 'f,
    U: Clone + 'f,
    A: Clone + 'f,
    B: Clone + 'f,
{
    fn from(value: F) -> Self {
        Handler::new(value)
    }
}

impl<'f, A, B, U, V> From<Handler<'f, A, B, U, V>> for Box<dyn HandlerFn<'f, A, B, U, V> + 'f>
where
    V: Clone + 'f,
    U: Clone + 'f,
    A: Clone + 'f,
    B: Clone + 'f,
{
    fn from(value: Handler<'f, A, B, U, V>) -> Self {
        value.0
    }
}
