use std::{cell::RefCell, rc::Rc};

use crate::{Ability, And, Fx, Handler};

pub type RcRead<'f, T> = Ability<'f, (), RcState<T>, T>;
pub type RcWrite<'f, T> = Ability<'f, T, RcState<T>, ()>;

#[derive(Clone)]
pub struct RcState<T>(Rc<RefCell<T>>);
impl<'f, T: Clone> RcState<T> {
    pub fn state_handler<B, O>(t: T) -> Handler<'f, And<Self, B>, B, O, (O, T)>
    where
        O: Clone,
        T: Copy + 'f,
        B: Clone,
    {
        Handler::new(move |e: Fx<'f, And<Self, B>, O>| {
            e.with_env()
                .map(|(s, o)| (o, s.left().0.borrow().clone()))
                .provide_left(Self(Rc::new(RefCell::new(t))))
        })
    }

    pub fn read() -> Fx<'f, And<RcRead<'f, T>, RcState<T>>, T> {
        RcRead::request(())
    }

    pub fn read_handler<B: Clone, V: Clone>() -> Handler<'f, And<RcRead<'f, T>, B>, B, V, V> {
        RcRead::handler(|_| Fx::func(|s: RcState<T>| s.0.borrow().clone()))
    }

    pub fn write(value: T) -> Fx<'f, And<RcWrite<'f, T>, RcState<T>>, ()>
    where
        T: Copy,
    {
        RcWrite::request(value)
    }

    pub fn write_handler() -> Handler<'f, And<RcWrite<'f, T>, RcState<T>>, RcState<T>, (), ()> {
        RcWrite::handler(|v: T| Fx::func(move |s: RcState<T>| s.0.borrow_mut().clone_from(&v)))
    }
}
