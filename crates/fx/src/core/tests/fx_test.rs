use crate::{
    core::has_put::{Has, Put},
    kernel::fx::Fx,
};
use std::convert::identity;

// Common structs and trait implementations for all tests
#[derive(Clone, Debug, PartialEq)]
struct A(pub i32);
#[derive(Clone, Debug, PartialEq)]
struct B(pub i32);
#[derive(Clone, Debug, PartialEq)]
struct S {
    a: A,
    b: B,
}
impl Has<A> for S {
    fn get(self) -> A {
        self.a
    }
}
impl Put<A> for S {
    fn put(mut self, value: A) -> Self {
        self.a = value;
        self
    }
}
impl Has<B> for S {
    fn get(self) -> B {
        self.b
    }
}
impl Put<B> for S {
    fn put(mut self, value: B) -> Self {
        self.b = value;
        self
    }
}

#[test]
fn pure() {
    let e = Fx::pure(22);
    assert_eq!(e.eval(), 22)
}

#[test]
fn value() {
    let e = Fx::value(22);
    assert_eq!(e.eval(), 22)
}

#[test]
fn contra_map() {
    let e = Fx::pending(|s: String| Fx::value(s.chars().rev().collect::<String>()));
    let e = e.adapt(|n: usize| n.to_string(), |_, _, value| Fx::value(value));
    let e = e.provide(12);
    assert_eq!(e.eval(), "21".to_owned())
}

#[test]
fn provide() {
    let e = Fx::pending(|n: usize| Fx::value(n * 10));
    let e = e.provide(12);
    assert_eq!(e.eval(), 120);
}

#[test]
fn chain() {
    let e = Fx::pending(|n: usize| Fx::value(n * 10));
    let e = e.adapt(identity, |_, _, n: usize| Fx::value(n.to_string()));
    let e = e.provide(12);
    assert_eq!(e.eval(), "120".to_owned())
}

#[test]
fn adapt() {
    let e = Fx::pending(|n: usize| Fx::value(n * 10));
    let e = e.adapt(
        |s: String| s.len(),
        |_, _, n: usize| Fx::value(n.to_string()),
    );
    let e = e.provide("hello".to_owned());
    assert_eq!(e.eval(), "50".to_owned());
}

#[test]
fn func() {
    let e = Fx::func(|u: usize| u.to_string());
    let e = e.provide(12);
    assert_eq!(e.eval(), "12".to_owned())
}

#[test]
fn test_and_then_chains_fx_with_different_state() {
    // S = i32, T = &'static str, P = (i32, &'static str)
    let fx1: Fx<'_, i32, i32> = Fx::value(10);
    let fx2: Fx<'_, &'static str, &'static str> = Fx::value("hello");
    let result_fx: Fx<'_, (i32, &'static str), &'static str> = fx1.and_then(fx2);
    let result = result_fx.provide((42, "world")).eval();
    // fx2 always returns "hello"
    assert_eq!(result, "hello");
}

#[test]
fn flat_map() {
    // S = usize, T = String, P = (usize, &'static str)
    let fx1: Fx<'_, usize, usize> = Fx::value(5);
    let fx2 = fx1.flat_map(|n| Fx::value(format!("{} apples", n)));
    // Provide a tuple as state, as required by the Pair trait
    let result = fx2.provide((5, ())).eval();
    assert_eq!(result, "5 apples");
}

#[test]
fn lifts_fx_to_larger_context() {
    let fx_a: Fx<A, i32> = Fx::pending(|a: A| Fx::value(a.0 + 1));
    let fx_b: Fx<B, i32> = Fx::pending(|b: B| Fx::value(b.0));
    // This should work: lift to anything that Has<A>, Has<B> respectively.
    let lifted_a: Fx<S, i32> = fx_a.lift();
    let lifted_b: Fx<S, i32> = fx_b.lift();
    let s = S { a: A(41), b: B(3) };
    assert_eq!(lifted_a.provide(s.clone()).eval(), 42);
    assert_eq!(lifted_b.provide(s.clone()).eval(), 3);
}

#[test]
fn zip_and_zip_left_and_zip_right() {
    #[derive(Clone, Debug, PartialEq)]
    struct S(i32);
    let fx1: Fx<S, i32> = Fx::pending(|s: S| Fx::value(s.0 + 1));
    let fx2: Fx<S, i32> = Fx::pending(|s: S| Fx::value(s.0 * 2));
    let s = S(10);
    // zip returns a tuple of both results
    let zipped = fx1.clone().zip(fx2.clone());
    assert_eq!(zipped.provide(s.clone()).eval(), (11, 20));
    // zip_left returns the result of the first
    let zipped_left = fx1.clone().zip_left(fx2.clone());
    assert_eq!(zipped_left.provide(s.clone()).eval(), 11);
    // zip_right returns the result of the second
    let zipped_right = fx1.zip_right(fx2);
    assert_eq!(zipped_right.provide(s).eval(), 20);
}

#[test]
fn zip_lifted_fx_on_struct_with_has_a_and_b() {
    let fx_a: Fx<A, i32> = Fx::pending(|a: A| Fx::value(a.0 + 1));
    let fx_b: Fx<B, i32> = Fx::pending(|b: B| Fx::value(b.0));
    let lifted_a: Fx<S, i32> = fx_a.lift();
    let lifted_b: Fx<S, i32> = fx_b.lift();
    let s = S { a: A(41), b: B(3) };
    let zipped = lifted_a.zip(lifted_b);
    assert_eq!(zipped.provide(s).eval(), (42, 3));
}

#[test]
fn has_pending_tuple() {
    #[derive(Clone)]
    struct N(i32);
    impl Has<N> for (N, ()) {
        fn get(self) -> N {
            self.0
        }
    }

    // Tuple context
    let fx = Fx::has_pending(|x: N| Fx::value(x.0 + 1));
    let result = fx.provide((N(2), ())).eval();
    assert_eq!(result, 3);
}

#[test]
fn has_pending_struct() {
    // Struct context
    #[derive(Clone)]
    struct Ctx {
        x: i32,
    }
    impl Has<i32> for Ctx {
        fn get(self) -> i32 {
            self.x
        }
    }
    let fx = Fx::has_pending(|x: i32| Fx::value(x * 2));
    let result = fx.provide(Ctx { x: 7 }).eval();
    assert_eq!(result, 14);
}

#[test]
fn has_pending_composed() {
    #[allow(dead_code)]
    #[derive(Clone)]
    struct Ctx {
        x: i32,
        y: i32,
    }
    impl Has<i32> for Ctx {
        fn get(self) -> i32 {
            self.x
        }
    }
    // Compose two has_pending calls
    let fx = Fx::has_pending(|x: i32| Fx::has_pending(move |y: i32| Fx::value(x + y)));
    let result = fx.provide(Ctx { x: 3, y: 4 }).eval();
    assert_eq!(result, 6); // Only x is used, y is ignored (since Has<i32> is implemented only for x)
}

#[test]
fn lift_map_composes_effects_with_hasput() {
    let a: Fx<A, A> = Fx::func(|a: A| A(a.0 + 1));
    let b = |a: A| Fx::func(move |b: B| ((a.0 + b.0) * 2));
    let composed: Fx<S, i32> = a.lift_map(b);
    let s = S { a: A(10), b: B(7) };
    assert_eq!(composed.provide(s).eval(), 36); // ((a + 1) + b) * 2
}
