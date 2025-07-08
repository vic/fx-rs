use fx::Fx;

#[test]
fn chain() {
    // Kernel Fx does not have adapt/provide, so we use immediate and eval
    let e = Fx::immediate((), "120".to_owned());
    let v = e.eval();
    assert_eq!(v, "120".to_owned());
}
