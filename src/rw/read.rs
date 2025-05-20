use crate::{Fx, Nil, ReqFx};

pub type Read<'f, T> = ReqFx<'f, Nil, Nil, &'f T>;
impl<'f, T> Read<'f, T> {
    pub fn read() -> Self::Fx<&'f T>
    where
        T: Clone + 'f,
    {
        ReqFx::request(Nil)
    }

    pub fn reader<F>(f: F) -> Self::Handler
    where
        F: Fn() -> &'f T + Copy + 'f,
    {
        Self::handler(move |_: Nil| Fx::pure(f()))
    }
}
