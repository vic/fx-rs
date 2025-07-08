use fx::Fx;

#[test]
fn adapt_contramap() {
    let e = Fx::pending(|i: u8| Fx::immediate(i, i * 10));
    let e = e.adapt(|_: ()| 2, |u, _, v| Fx::immediate(u, v));
    let v = e.eval();
    assert_eq!(v, 20)
}
