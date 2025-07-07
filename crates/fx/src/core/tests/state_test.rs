// Unit tests for core/state.rs functionality
#[cfg(test)]
mod state_tests {
    use crate::{Fx, State};

    #[test]
    fn ctx() {
        let e = State::get();
        let e = e.provide(22);
        assert_eq!(e.eval(), 22);
    }

    #[test]
    fn ctx_nested() {
        let e = State::get().map_m(|_| State::get());
        let e = e.provide(22);
        assert_eq!(e.eval(), 22);
    }

    #[test]
    fn map() {
        let e = State::get().map(|n: usize| n * 10);
        let e = e.provide(12);
        assert_eq!(e.eval(), 120)
    }

    #[test]
    fn flat_map_basic() {
        let fx = Fx::value(2).flat_map(|x| Fx::value(x * 10));
        assert_eq!(fx.provide(((), ())).eval(), 20);
    }

    #[test]
    fn flat_map_ctx() {
        let fx: Fx<(u8, u16), u16> =
            Fx::func(|u: u8| u + 1).flat_map(|u: u8| Fx::func(move |v: u16| (u as u16) + v));
        assert_eq!(fx.provide((1, 2)).eval(), 4);
    }
}
