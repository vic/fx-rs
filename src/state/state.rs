use dyn_clone::{DynClone, clone_trait_object};
use std::{cell::RefCell, rc::Rc};

use crate::{Ability, Fx, Handler};

clone_trait_object!(<T> ST<T>);
pub trait ST<T>: DynClone {
    fn read(&self) -> T;
    fn write(&self, t: T);
}

#[derive(Clone)]
pub struct State<'f, T>(Box<dyn ST<T> + 'f>);

pub type Read<'f, T> = Ability<'f, (), State<'f, T>, T>;
pub type Write<'f, T> = Ability<'f, T, State<'f, T>, ()>;

#[derive(Clone)]
pub struct RcState<T: Clone>(Rc<RefCell<T>>);
impl<'f, T: Clone> ST<T> for RcState<T> {
    fn read(&self) -> T {
        self.0.borrow().clone()
    }

    fn write(&self, t: T) {
        self.0.borrow_mut().clone_from(&t)
    }
}

impl<'f, T: Clone> State<'f, T> {
    pub fn rc_handler<B, O>(t: T) -> Handler<'f, (Self, B), B, O, (O, T)>
    where
        O: Clone,
        T: Clone + 'f,
        B: Clone,
    {
        Handler::new(move |e: Fx<'f, (Self, B), O>| {
            let s = State(Box::new(RcState(Rc::new(RefCell::new(t.clone())))));
            e.from_env().map(|(n, o)| (o, n.0.0.read())).provide_left(s)
        })
    }

    pub fn read() -> Fx<'f, (Read<'f, T>, State<'f, T>), T> {
        Read::request(())
    }

    pub fn reader<B: Clone, V: Clone>() -> Handler<'f, (Read<'f, T>, B), B, V, V> {
        Read::handler(|_| Fx::func(|s: State<T>| s.0.read()))
    }

    pub fn write(value: T) -> Fx<'f, (Write<'f, T>, State<'f, T>), ()>
    where
        T: Clone,
    {
        Write::request(value)
    }

    pub fn writer<B: Clone>() -> Handler<'f, (Write<'f, T>, B), B, (), ()> {
        Write::handler(|v: T| Fx::func(move |s: State<'f, T>| s.0.write(v.clone())))
    }
}
