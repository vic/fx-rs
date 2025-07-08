use fx::Fx;

#[test]
fn func() {
    let e = Fx::immediate((), "12".to_owned());
    let v = e.eval();
    assert_eq!(v, "12".to_owned());
}
