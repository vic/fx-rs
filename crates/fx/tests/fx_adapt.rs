use fx::Fx;

#[test]
fn adapt() {
    // Kernel Fx does not have adapt/provide, so we use immediate and eval
    let e = Fx::immediate((), "50".to_owned());
    let v = e.eval();
    assert_eq!(v, "50".to_owned());
}
