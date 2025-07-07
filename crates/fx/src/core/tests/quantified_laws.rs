use crate::State;
use std::fmt::Debug;

// Law: get/set roundtrip for State effect
fn law_get_set_roundtrip<T>(init: T)
where
    T: Debug + Clone + PartialEq,
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
    T: Debug + Clone + PartialEq,
{
    let fx = State::set(set_val.clone()).then(State::set(set_val.clone())).then(State::get());
    let result = fx.provide(init).eval();
    assert_eq!(result, set_val);
}

#[test]
fn test_state_set_idempotent() {
    law_set_idempotent(0, 99);
    law_set_idempotent(String::from("a"), String::from("b"));
}
