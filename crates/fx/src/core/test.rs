use std::convert::identity;

use crate::{Ability, Fx, Lens, State};

#[test]
fn value() {
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
    let e = e.adapt(|n: usize| n.to_string(), |_, _, value| Fx::value(value));
    let e = e.provide(12);
    assert_eq!(e.eval(), "21".to_owned())
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

#[test]
fn put() {
    let e = State::set(22).map(|n: usize| n * 10);
    let e = e.provide(99);
    assert_eq!(e.eval(), 220)
}

#[test]
fn put_nested() {
    let e = State::set(22)
        .map_m(|n: usize| State::set(n * 10))
        .map_m(|_| State::get());
    let e = e.provide(99);
    assert_eq!(e.eval(), 220)
}

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

#[test]
fn fold_outcome() {
    let e = Ability::request(11).then(Ability::request(22));
    let h =
        Ability::new(|n: usize| Fx::value(n * 2)).acc_outcome_with(0, |acc, i| Fx::value(acc + i));
    let e = h.handle(e);
    let v = e.eval();

    assert_eq!(v, (66, 44))
}

#[test]
fn option_outcome() {
    let e = Ability::request(12)
        .map(|s: String| s.chars().rev().collect::<String>())
        .map(|s: String| s.parse::<usize>().unwrap());
    let ab = Ability::new(|n: usize| Fx::value(n.to_string()));
    let h = ab.acc_outcome_default();
    let e = h.handle(e);
    let v = e.eval();

    assert_eq!(v, (Some("12".to_owned()), 21))
}

#[test]
fn some_outcome() {
    let e = Ability::request(12)
        .map(|s: String| s.chars().rev().collect::<String>())
        .map(|s: String| s.parse::<usize>().unwrap());
    let h = Ability::new(|n: usize| Fx::value(n.to_string()))
        .acc_outcome_default()
        .map(|e| e.map(|(o, u): (Option<String>, usize)| (o.unwrap(), u)));
    let e = h.handle(e);
    let v = e.eval();

    assert_eq!(v, ("12".to_owned(), 21))
}

#[test]
fn vec_outcome() {
    let e = Ability::request(11).then(Ability::request(22));
    let h = Ability::new(|n: usize| Fx::value(n * 2)).acc_outcome_default();
    let e = h.handle(e);
    let v = e.eval();

    assert_eq!(v, (vec![22, 44], 44))
}

#[test]
fn acc_outcome() {
    type F<'f> = Ability<'f, u8, (), u8>;
    let x: F = F::new(|n| Fx::value(n + 10));
    let y: F = F::new(|n| Fx::value(n * 2));
    let e = Ability::request(x).then(Ability::request(y)).map(|_| true);

    let h = Ability::new(Fx::value).acc_outcome_with(2u8, |acc, f: F| f.apply(acc));

    let v = e.via(h).eval();
    assert_eq!(v, (24u8, true))
}

#[derive(Clone, Debug, PartialEq)]
struct ST {
    a: i32,
    b: String,
}
impl ST {
    fn b<'f>() -> Lens<'f, ST, String> {
        Lens::new(|s: ST| s.b, |s, b| ST { b, ..s })
    }

    fn a<'f>() -> Lens<'f, ST, i32> {
        Lens::new(|s: ST| s.a, |s, a| ST { a, ..s })
    }
}

#[test]
fn test_focus_out() {
    let e: Fx<String, ()> = State::set("hello".to_string()).then(Fx::value(()));

    let e: Fx<ST, ()> = e.via(ST::b().zoom_out());

    let e = e.then(State::get());

    let result = e
        .provide(ST {
            a: 0,
            b: "world".to_string(),
        })
        .eval();

    assert_eq!(
        result,
        ST {
            a: 0,
            b: "hello".to_string()
        }
    );
}

#[test]
fn test_focus_in() {
    let e: Fx<ST, ()> = Fx::immediate(
        ST {
            a: 42,
            b: "hello".to_string(),
        },
        (),
    );

    let e: Fx<ST, ()> = e.via(ST::b().zoom_in(
        |()| State::set("bye".to_string()).map(|_| ()), // inner effect updates
    ));

    let e = e.then(State::get());

    let result = e
        .provide(ST {
            a: 0,
            b: "bad".to_string(),
        })
        .eval();
    assert_eq!(
        result,
        ST {
            a: 42,
            b: "bye".to_string()
        }
    );
}

#[test]
fn test_focus_in_and_out() {
    let inner: Fx<i32, ()> = Fx::immediate(10, ());

    let outer: Fx<ST, ()> = inner
        .via(ST::a().zoom_out())
        .then(State::map(|s: ST| ST { a: s.a * 2, ..s }))
        .then(Fx::value(()));

    let back: Fx<ST, i32> = outer.via(ST::a().zoom_in(|_| State::<i32>::map(|n| n + 10)));

    let e = back.then(State::get());

    let result = e
        .provide(ST {
            a: 0,
            b: "hello".to_owned(),
        })
        .eval();

    assert_eq!(
        result,
        ST {
            a: 30,
            b: "hello".to_owned()
        }
    );
}
