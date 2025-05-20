use crate::{Fx, Nil, ReqFx};

#[test]
fn test_apply_ctx_fn() {
    let e = Fx::apply("hello");
    let v = e.provide(|s: &str| s.len()).eval();
    assert_eq!(v, Some("hello".len()))
}

#[test]
fn test_request() {
    let e = Fx::request("hello");
    let v = e.provide_left(|s: &str| Fx::pure(s.len())).eval();
    assert_eq!(v, Some("hello".len()))
}

#[test]
fn test_handler() {
    let e = Fx::request("hello");
    let handler = Fx::handler(|s: &str| Fx::pure(s.len()));
    let v = handler.handle(e).eval();
    assert_eq!(v, Some("hello".len()))
}

#[test]
fn test_handle() {
    let e = Fx::handle("hello");
    let handler = Fx::handler(|s: &str| Fx::pure(s.len()));
    let v = e.provide_left(handler).eval();
    assert_eq!(v, Some("hello".len()))
}

#[test]
fn test_request_fx() {
    type StrLenReq<'f> = ReqFx<'f, &'f str, Nil, usize>;
    let e: StrLenReq::Fx<usize> = StrLenReq::request("hello");
    let handler: StrLenReq::Handler = StrLenReq::handler(|s: &str| Fx::pure(s.len()));
    let v = e.provide_left(handler).eval();
    assert_eq!(v, Some("hello".len()))
}
