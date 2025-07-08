use fx::Fx;

#[test]
fn value() {
    let e = Fx::immediate((), 22);
    let v = e.eval();
    assert_eq!(v, 22)
}
