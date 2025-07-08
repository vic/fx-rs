use crate::kernel::fx::Fx;
use std::convert::identity;

#[test]
fn pure() {
    let e = Fx::pure(22);
    assert_eq!(e.eval(), 22)
}

#[test]
fn value() {
    let e = Fx::value(22);
    assert_eq!(e.eval(), 22)
}

#[test]
fn contra_map() {
    let e = Fx::pending(|s: String| Fx::value(s.chars().rev().collect::<String>()));
    let e = e.adapt(|n: usize| n.to_string(), |_, _, value| Fx::value(value));
    let e = e.provide(12);
    assert_eq!(e.eval(), "21".to_owned())
}

#[test]
fn provide() {
    let e = Fx::pending(|n: usize| Fx::value(n * 10));
    let e = e.provide(12);
    assert_eq!(e.eval(), 120);
}

#[test]
fn chain() {
    let e = Fx::pending(|n: usize| Fx::value(n * 10));
    let e = e.adapt(identity, |_, _, n: usize| Fx::value(n.to_string()));
    let e = e.provide(12);
    assert_eq!(e.eval(), "120".to_owned())
}

#[test]
fn adapt() {
    let e = Fx::pending(|n: usize| Fx::value(n * 10));
    let e = e.adapt(
        |s: String| s.len(),
        |_, _, n: usize| Fx::value(n.to_string()),
    );
    let e = e.provide("hello".to_owned());
    assert_eq!(e.eval(), "50".to_owned());
}

#[test]
fn func() {
    let e = Fx::func(|u: usize| u.to_string());
    let e = e.provide(12);
    assert_eq!(e.eval(), "12".to_owned())
}

#[test]
fn test_and_then_chains_fx_with_different_state() {
    // S = i32, T = &'static str, P = (i32, &'static str)
    let fx1: Fx<'_, i32, i32> = Fx::value(10);
    let fx2: Fx<'_, &'static str, &'static str> = Fx::value("hello");
    let result_fx: Fx<'_, (i32, &'static str), &'static str> = fx1.and_then(fx2);
    let result = result_fx.provide((42, "world")).eval();
    // fx2 always returns "hello"
    assert_eq!(result, "hello");
}
