use super::{Fx, Nil};

#[test]
fn eval_constant() {
    let e = Fx::immediate(22);
    let v = e.eval();
    assert_eq!(v, Some(22))
}

#[test]
fn eval_stopped() {
    let s = Fx::stopped(|| Fx::immediate(22));
    assert_eq!(s.eval(), None)
}

#[test]
fn eval_pending() {
    let s = Fx::pending(|_: Nil| Fx::immediate(22));
    assert_eq!(s.eval(), Some(22))
}
