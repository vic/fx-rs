use crate::State;
use std::fmt::Debug;

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

fn law_set_idempotent<T>(init: T, set_val: T)
where
    T: Debug + Clone + PartialEq,
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

fn law_state_acc_commute(init_state: i32, init_acc: Vec<i32>, input: i32) {
    use crate::Fx;
    let fx = Fx::pending(|(_state, mut acc): (i32, Vec<i32>)| {
        let state = input;
        let s2 = state * 2;
        acc.push(input);
        Fx::immediate((state, acc), s2)
    });
    let result1 = fx.clone().provide((init_state, init_acc.clone())).eval();
    let swapped_fx = fx.contra_map(|(acc, state)| (state, acc), |_, (state, acc)| (acc, state));
    let result2 = swapped_fx.provide((init_acc, init_state)).eval();
    assert_eq!(result1, result2);
}

#[test]
fn test_state_acc_commute() {
    law_state_acc_commute(10, vec![1, 2], 5);
}

#[derive(Clone, Debug, PartialEq)]
struct NestedCtx {
    state: i32,
    acc: Vec<i32>,
}

fn law_struct_nested_quantification(init_state: i32, init_acc: Vec<i32>, input: i32) {
    use crate::Fx;
    let fx = Fx::pending(|mut ctx: NestedCtx| {
        ctx.state = input;
        ctx.acc.push(input);
        let s2 = ctx.state * 2;
        Fx::immediate(ctx, s2)
    });
    let ctx = NestedCtx {
        state: init_state,
        acc: init_acc.clone(),
    };
    let result = fx.provide(ctx.clone()).eval();
    // Manual reference result
    let mut expected_ctx = ctx.clone();
    expected_ctx.state = input;
    expected_ctx.acc.push(input);
    let expected = expected_ctx.state * 2;
    assert_eq!(result, expected);
}

#[test]
fn test_struct_nested_quantification() {
    law_struct_nested_quantification(10, vec![1, 2], 5);
}
