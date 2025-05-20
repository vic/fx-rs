use crate::Fx;

#[test]
fn test_apply_ctx_fn() {
    let e = Fx::apply("hello");
    let v = e.provide(|s: &str| s.len()).eval();
    assert_eq!(v, Some("hello".len()))
}

#[test]
fn test_suspend() {
    let e = Fx::suspend("hello");
    let v = e.provide_left(|s: &str| Fx::pure(s.len())).eval();
    assert_eq!(v, Some("hello".len()))
}

#[test]
fn test_handler() {
    let e = Fx::suspend("hello");
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
