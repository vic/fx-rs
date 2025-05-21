use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use crate::{Ability, And, Fx, Handler};

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
            e.with_env()
                .map(|(s, o)| (o, s.left().0.borrow().clone()))
                .provide_left(Self(Rc::new(RefCell::new(t))))
        })
    }
}

pub struct Read<'f, T: Clone>(PhantomData<ReadCap<'f, T>>);
type ReadCap<'f, T> = Ability<'f, (), State<T>, T>;

impl<'f, T: Clone> Read<'f, T> {
    pub fn read() -> Fx<'f, And<ReadCap<'f, T>, State<T>>, T> {
        ReadCap::request(())
    }

    pub fn handler<B: Clone, V: Clone>() -> Handler<'f, And<ReadCap<'f, T>, B>, B, V, V> {
        ReadCap::handler(|_| Fx::func(|s: State<T>| s.0.borrow().clone()))
    }
}

pub struct Write<'f, T: Clone>(PhantomData<WriteCap<'f, T>>);
type WriteCap<'f, T> = Ability<'f, T, State<T>, ()>;

impl<'f, T: Copy> Write<'f, T> {
    pub fn write(value: T) -> Fx<'f, And<WriteCap<'f, T>, State<T>>, ()> {
        WriteCap::request(value)
    }

    pub fn handler() -> Handler<'f, And<WriteCap<'f, T>, State<T>>, State<T>, (), ()> {
        WriteCap::handler(|v: T| Fx::func(move |s: State<T>| s.0.borrow_mut().clone_from(&v)))
    }
}
