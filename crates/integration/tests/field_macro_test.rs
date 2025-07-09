use fx::{Fx, State};
use fx_do::fx_do;

#[derive(Clone, Debug, PartialEq)]
struct A(u8);
#[derive(Clone, Debug, PartialEq)]
struct B(u8);
#[derive(Clone, Debug, PartialEq, fx_field::HasFields)]
struct Ctx {
    a: A,
    b: B,
}

#[test]
fn test_struct_field() {
    let fx = fx_do!({
        let a: A = State::get().same();
        let b: B = State::get().same();
        a.0 + b.0
    });
    let result = fx.provide(Ctx { a: A(10), b: B(10) }).eval();
    assert_eq!(result, 20);
}
