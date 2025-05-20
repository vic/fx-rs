use crate::{Fx, Nil, ReqFx};

pub type Write<'f, T> = ReqFx<'f, T, Nil, ()>;
impl<'f, T> Write<'f, T> {
    pub fn write(t: T) -> Self::Fx<()>
    where
        T: Copy + 'f,
    {
        Self::request(t)
    }

    //     pub fn writer<F>(f: F) -> Self::Handler
    //     where
    //         F: Fn(T) -> () + Copy + 'f,
    //         T: Clone
    //     {
    //         Self::handler(move |t: T| Fx::pure(f(t)))
    //     }
}
