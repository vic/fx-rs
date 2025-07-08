use fx::Fx;

#[test]
fn eval_pending() {
    let e = Fx::pending(|u: ()| Fx::immediate(u, 22));
    let v = e.eval();
    assert_eq!(v, 22)
}
