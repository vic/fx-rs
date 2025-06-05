use std::convert::identity;

use super::{Ctx, Fx};

#[test]
fn immediate() {
    let e = Fx::value(22);
    assert_eq!(e.eval(), 22)
}

#[test]
fn provide() {
    let e = Fx::pending(|n: usize| Fx::value(n * 10));
    let e = e.provide(12);
    assert_eq!(e.eval(), 120);
}

#[test]
fn func() {
    let e = Fx::func(|u: usize| u.to_string());
    let e = e.provide(12);
    assert_eq!(e.eval(), "12".to_owned())
}

#[test]
fn contra_map() {
    let e = Fx::pending(|s: String| Fx::value(s.chars().rev().collect::<String>()));
    let e = e.adapt(|n: usize| n.to_string(), |_ctx, value| Fx::value(value));
    let e = e.provide(12);
    assert_eq!(e.eval(), "21".to_owned())
}

#[test]
fn chain() {
    let e = Fx::pending(|n: usize| Fx::value(n * 10));
    let e = e.adapt(identity, |_ctx, n: usize| Fx::value(n.to_string()));
    let e = e.provide(12);
    assert_eq!(e.eval(), "120".to_owned())
}

#[test]
fn adapt() {
    let e = Fx::pending(|n: usize| Fx::value(n * 10));
    let e = e.adapt(|s: String| s.len(), |_, n: usize| Fx::value(n.to_string()));
    let e = e.provide("hello".to_owned());
    assert_eq!(e.eval(), "50".to_owned());
}

#[test]
fn ctx() {
    let e = Ctx::get();
    let e = e.provide(22);
    assert_eq!(e.eval(), 22);
}

#[test]
fn ctx_nested() {
    let e = Ctx::get().map_m(|_| Ctx::get());
    let e = e.provide(22);
    assert_eq!(e.eval(), 22);
}

#[test]
fn map() {
    let e = Ctx::get().map(|n: usize| n * 10);
    let e = e.provide(12);
    assert_eq!(e.eval(), 120)
}

#[test]
fn put() {
    let e = Ctx::set(22).map(|n: usize| n * 10);
    let e = e.provide(99);
    assert_eq!(e.eval(), 220)
}

#[test]
fn put_nested() {
    let e = Ctx::set(22)
        .map_m(|n: usize| Ctx::set(n * 10))
        .map_m(|_| Ctx::get());
    let e = e.provide(99);
    assert_eq!(e.eval(), 220)
}
