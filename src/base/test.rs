use crate::{Ability, Fx, Nil};

#[test]
#[should_panic]
fn start_halted() {
    let e = Fx::<Nil, String>::halted().start(|c| c);
    e.eval();
}

#[test]
fn eval_func() {
    let e = Fx::func(|s: &str| s.len()).and_nil();
    let v = e.provide_left("hello").eval();
    assert_eq!(v, Some("hello".len()))
}

#[test]
fn test_apply_ctx_fn() {
    let e = Fx::apply("hello");
    let v = e.provide(|s: &str| s.len()).eval();
    assert_eq!(v, Some("hello".len()))
}

#[test]
fn test_request_fx() {
    type StrLenReq<'f> = Ability<'f, &'f str, Nil, usize>;
    let e = StrLenReq::request("hello");
    let handler = StrLenReq::handler(|s: &str| Fx::pure(s.len()));
    let v = e.handle(handler).eval();
    assert_eq!(v, Some("hello".len()))
}

#[test]
fn contramap() {
    let e = Fx::func(|v: usize| v.to_string())
        .from_env()
        .contra_map(|n: usize| n * 2)
        .provide(10)
        .eval();
    assert_eq!(e, Some((20, "20".to_owned())))
}

#[test]
fn test_env() {
    let e = Fx::func(|n: usize| n.to_string())
        .into_env()
        .provide_left(20)
        .contra_map(|s: String| s.chars().rev().collect())
        .from_env()
        .provide("boom".to_owned())
        .eval();
    assert_eq!(
        e,
        Some(("boom".to_owned(), ("20".to_owned(), "moob".to_owned())))
    )
}
