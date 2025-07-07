use fx::{Has, Fx, State};
use fx_do::fx_do;

#[test]
fn test_letm_macro_compat() {
    let fx = fx_do!({
        letm!(x = Fx::value(2));
        letm!(y = Fx::value(3));
        x + y
    });
    let result = fx.eval();
    assert_eq!(result, 5);
}

#[test]
fn test_letf_macro_compat() {
    let fx = fx_do!({
        let x = 2;
        letf!(y = Fx::func(move |_| x + 3));
        y
    });
    let result = fx.provide(((), ())).eval();
    assert_eq!(result, 5);
}

#[test]
fn test_map_m() {
    let fx = fx_do!({
        let x = 2;
        let y = 3;
        x + y
    });
    let result = fx.eval();
    assert_eq!(result, 5);
}

#[test]
fn test_contra_map() {
    let fx = fx_do!({
        let x = 2;
        let y = Fx::value(3).contra_map(|_: u8| (), |u, _| u).same();
        let z = x * y;
        z
    });
    let result = fx.provide(10).eval();
    assert_eq!(result, 6);
}

#[test]
fn test_state() {
    let fx = fx_do!({
        let x: u8 = State::get().same();
        x * x
    });
    let result = fx.provide(10).eval();
    assert_eq!(result, 100);
}

#[derive(Clone, Debug, PartialEq)]
struct A(u8);
#[derive(Clone, Debug, PartialEq)]
struct B(u8);
#[derive(Clone, Debug, PartialEq)]
struct Ctx {
    a: A,
    b: B,
}

impl Has<A> for Ctx {
    fn get(&self) -> &A {
        &self.a
    }
}
impl Has<B> for Ctx {
    fn get(&self) -> &B {
        &self.b
    }
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

impl Has<A> for (A, B) {
    fn get(&self) -> &A {
        &self.0
    }
}
impl Has<B> for (A, B) {
    fn get(&self) -> &B {
        &self.1
    }
}

#[test]
fn test_tuple_field() {
    let fx_tuple = fx_do!({
        let a: A = State::get().same();
        let b: B = State::get().same();
        a.0 + b.0
    });
    let result_tuple = fx_tuple.provide((A(10), B(10))).eval();
    assert_eq!(result_tuple, 20);
}

#[test]
fn flat_map_ctx_do_macro() {
    let fx: Fx<(u8, u16), u16> = fx_do!({
        let u = Fx::func(|u: u8| u + 1).bind();
        let v = Fx::func(move |v: u16| (u as u16) + v).same();
        v
    });
    assert_eq!(fx.provide((1u8, 2u16)).eval(), 4);
}

#[test]
fn test_letf_flat_map() {
    let fx = fx_do!({
        let x = 2;
        let y = Fx::func(move |_| x + 3).bind();
        y
    });
    let result = fx.provide(((), ())).eval();
    assert_eq!(result, 5);
}

#[test]
fn test_letm_map_m() {
    let fx = fx_do!({
        let x = 2;
        let y = x + 3;
        y
    });
    let result = fx.eval();
    assert_eq!(result, 5);
}
