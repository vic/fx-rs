// Unit tests for core/ability.rs and handler.rs functionality
#[cfg(test)]
mod ability_tests {
    use crate::{Ability, Fx, State};

    #[test]
    fn modify_and_continue_delimited_continuation() {
        let e = Ability::request::<(_, _)>(5).map(|n| n * 2);
        let h = Ability::new(|i| Fx::value(i + 10)).handler();

        let e = h.handle(e);
        let v = e.eval();

        // Expected: (5 (ability input) + 10 (handler)) * 2 (continuation) = 30
        assert_eq!(v, 30)
    }

    #[test]
    fn ctx_map_ability() {
        let e = Ability::request::<(_, _)>(1u8).then(Ability::request(2u8));
        let ab = Ability::new(|i: u8| State::map(move |n: u8| n + i));
        let v = e.via(ab.handler()).provide(10).eval();
        assert_eq!(v, 13)
    }

    #[test]
    fn rw_ability() {
        let read = State::<u8>::get();
        let write = State::<u8>::map(|n| n * 10);
        let e = Ability::request::<(_, _)>(read).then(Ability::request(write));
        let ab = Ability::new(|fx| fx);
        let v = e.via(ab.handler()).provide(22).eval();
        assert_eq!(v, 220)
    }
}
