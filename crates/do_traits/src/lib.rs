pub trait Same<V>: Sized {
    /// This method should not be invoked directly. It is intended to be used within the `fx_do!` macro.
    fn same(self) -> V {
        unreachable!(
            "The `same` method cannot be invoked directly. Use it within the `fx_do!` macro."
        );
    }
}

pub trait Bind<V>: Sized {
    /// This method should not be invoked directly. It is intended to be used within the `fx_do!` macro.
    fn bind(self) -> V {
        unreachable!(
            "The `bind` method cannot be invoked directly. Use it within the `fx_do!` macro."
        );
    }
}

pub use fx::*;

impl<'f, S: Clone, V: Clone> Same<V> for Fx<'f, S, V> {}
impl<'f, S: Clone, V: Clone> Bind<V> for Fx<'f, S, V> {}
