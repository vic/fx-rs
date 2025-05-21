use std::{cell::RefCell, rc::Rc};

use crate::{And, Cap, Fx, Handler};

#[derive(Clone)]
pub struct State<T>(Rc<RefCell<T>>);
impl<'f, T> State<T> {
    pub fn handler<B, O>(t: T) -> Handler<'f, And<Self, B>, B, O, (O, T)>
    where
        O: Clone,
        T: Copy + 'f,
        B: Clone,
    {
        Handler::new(move |e: Fx<'f, And<Self, B>, O>| {
            e.left_down()
                .map(|(s, o)| (o, s.0.borrow().clone()))
                .provide_left(Self(Rc::new(RefCell::new(t))))
        })
    }
}

pub type Read<'f, T> = Cap<'f, (), State<T>, T>;
impl<'f, T: Clone> Read<'f, T> {
    pub fn read() -> Fx<'f, And<Self, State<T>>, T> {
        Self::request(())
    }

    pub fn reader<B: Clone, V: Clone>() -> Handler<'f, And<Self, B>, B, V, V> {
        Self::new_handler(|_| Fx::func(|s: State<T>| s.0.borrow().clone()))
    }
}

pub type Write<'f, T> = Cap<'f, T, State<T>, ()>;
impl<'f, T: Copy> Write<'f, T> {
    pub fn write(value: T) -> Fx<'f, And<Self, State<T>>, ()> {
        Self::request(value)
    }

    pub fn writer() -> Handler<'f, And<Self, State<T>>, State<T>, (), ()> {
        Self::new_handler(|v: T| Fx::func(move |s: State<T>| s.0.borrow_mut().clone_from(&v)))
    }
}
