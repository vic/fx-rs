use fx::Fx;

#[test]
fn provide() {
    let e = Fx::immediate((), 120);
    let v = e.eval();
    assert_eq!(v, 120);
}
