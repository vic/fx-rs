use crate::{Ability, Fx, Nil};

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
    let v = e.handle_left(handler).eval();
    assert_eq!(v, Some("hello".len()))
}
