use crate::Fx;

#[test]
fn ctx_fn_apply() {
    type StrLen = fn(&str) -> usize;
    let e = Fx::<StrLen, usize>::apply("hello");

    fn str_len(s: &str) -> usize {
        s.len()
    }
    let v = e.provide(str_len).eval();
    assert_eq!(v, Some("hello".len()))
}
