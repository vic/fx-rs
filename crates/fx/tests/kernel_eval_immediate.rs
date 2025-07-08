// Integration test for Fx::immediate and eval
use fx::Fx;

#[test]
fn eval_immediate() {
    let e = Fx::immediate((), 22);
    let v = e.eval();
    assert_eq!(v, 22)
}
