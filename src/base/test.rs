use crate::Fx;

type StrLen = fn(&str) -> usize;
fn str_len(s: &str) -> usize {
    s.len()
}

#[test]
fn test_apply() {
    let e = Fx::<StrLen, usize>::apply("hello");
    let p = e.provide(str_len);
    let v = p.eval();
    assert_eq!(v, Some("hello".len()))
}
