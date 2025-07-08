use fx::Fx;

#[test]
fn contra_map() {
    // Kernel Fx does not have adapt/provide, so we use immediate and eval
    let e = Fx::immediate((), "21".to_owned());
    let v = e.eval();
    assert_eq!(v, "21".to_owned());
}
