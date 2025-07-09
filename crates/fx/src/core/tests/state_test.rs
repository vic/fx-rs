use std::fmt::Debug;

use crate::core::state::State;
use crate::kernel::fx::Fx;

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

// Law: get/set roundtrip for State effect
fn law_get_set_roundtrip<T>(init: T)
where
    T: Clone + PartialEq + Debug,
{
    let fx = State::<T>::get();
    let s1 = fx.provide(init.clone()).eval();
    let fx2 = State::set(s1.clone()).then(State::get());
    let result = fx2.provide(s1.clone()).eval();
    assert_eq!(result, s1);
}

#[test]
fn test_state_get_set_roundtrip() {
    law_get_set_roundtrip(42);
    law_get_set_roundtrip(String::from("hello"));
}

// Law: idempotence of set for State effect
fn law_set_idempotent<T>(init: T, set_val: T)
where
    T: Clone + PartialEq + Debug,
{
    let fx = State::set(set_val.clone())
        .then(State::set(set_val.clone()))
        .then(State::get());
    let result = fx.provide(init).eval();
    assert_eq!(result, set_val);
}

#[test]
fn test_state_set_idempotent() {
    law_set_idempotent(0, 99);
    law_set_idempotent(String::from("a"), String::from("b"));
}

#[test]
fn update_increments_state() {
    let e = State::update::<_, (_, _), _>(|n: usize| Fx::pure(n + 1));
    let e = e.provide_left(10);
    let result = e.eval();
    assert_eq!(result, 11);

    let e = State::update::<_, (_, _), _>(|n: usize| Fx::pure(n + 1)).then(State::get());
    let e = e.provide_left(10);
    let result = e.eval();
    assert_eq!(result, (11, ()));
}
