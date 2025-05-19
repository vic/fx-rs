use super::{Fx, Nil};

#[test]
fn eval_constant() {
    let e = Fx::immediate(22);
    let v = e.eval();
    assert_eq!(v, Some(22))
}

#[test]
fn eval_func() {
    let e = Fx::func(|s: &str| s.len()).and_nil();
    let v = e.provide_left("hello").eval();
    assert_eq!(v, Some("hello".len()))
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

#[test]
#[should_panic]
fn start_halted() {
    let e = Fx::<Nil, String>::halted().start(|c| c);
    e.eval();
}
