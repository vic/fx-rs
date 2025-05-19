use crate::Fx;

#[test]
fn ctx_fn_apply() {
    let e = Fx::<&str, usize>::apply("hello");
    let v = e.provide(|s: &str| s.len()).eval();
    assert_eq!(v, Some("hello".len()))
}
