use super::fx::Fx;

#[test]
fn eval_immediate() {
    let e = Fx::immediate((), 22);
    let v = e.eval();
    assert_eq!(v, 22)
}

#[test]
fn eval_pending() {
    let e = Fx::pending(|u: ()| Fx::immediate(u, 22));
    let v = e.eval();
    assert_eq!(v, 22)
}

#[test]
fn adapt_contramap() {
    let e = Fx::pending(|i: u8| Fx::immediate(i, i * 10));
    let e = e.adapt(|_: ()| 2, |u, _, v| Fx::immediate(u, v));
    let v = e.eval();
    assert_eq!(v, 20)
}
