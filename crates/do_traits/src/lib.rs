// Same (map_m) and Bind (flat_map) traits for ergonomic effectful programming with fx_do!

pub trait Same<V>: Sized {
    fn same(self) -> V {
        unreachable!()
    }
}

pub trait Bind<V>: Sized {
    fn bind(self) -> V {
        unreachable!()
    }
}

use fx::Fx;

impl<'f, S: Clone, V: Clone> Same<V> for Fx<'f, S, V> {}
impl<'f, S: Clone, V: Clone> Bind<V> for Fx<'f, S, V> {}
