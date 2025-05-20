use crate::Fx;

#[test]
fn ctx_fn_apply() {
    let e = Fx::<&str, usize>::apply("hello");
    let v = e.provide(|s: &str| s.len()).eval();
    assert_eq!(v, Some("hello".len()))
}

#[test]
fn test_suspend() {
    let e = Fx::<&str, usize>::suspend("hello");
    let v = e.provide_left(|s: &str| Fx::immediate(s.len())).eval();
    assert_eq!(v, Some("hello".len()))
}
