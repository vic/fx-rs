use crate::{Has, Put, kernel::fx::Fx};

#[test]
fn provide_sets_state() {
    let fx = Fx::value(42);
    let fx2 = fx.provide::<()>(());
    let _ = fx2;
}

#[test]
fn provide_left_sets_left_part() {
    let fx: Fx<'static, (i32, i32), i32> = Fx::value(7);
    let fx2 = fx.provide_left::<i32, i32>(1);
    let _ = fx2;
}

#[derive(Clone, Debug, PartialEq)]
struct Ctx {
    a: i32,
    b: String,
}

impl Has<i32> for Ctx {
    fn get(self) -> i32 {
        self.a
    }
}
impl Put<i32> for Ctx {
    fn put(mut self, value: i32) -> Self {
        self.a = value;
        self
    }
}

impl Put<String> for Ctx {
    fn put(mut self, value: String) -> Self {
        self.b = value;
        self
    }
}

#[test]
fn provide_has_sets_field() {
    let ctx = Ctx {
        a: 0,
        b: "init".to_owned(),
    };
    let fx: Fx<Ctx, i32> = Fx::pending(|c: Ctx| Fx::value(c.a));
    let fx2 = fx.update_context(42);
    assert_eq!(fx2.provide(ctx.clone()).eval(), 42);
}

#[test]
fn provide_has_sets_string_field() {
    let ctx = Ctx {
        a: 7,
        b: "init".to_owned(),
    };
    let fx: Fx<Ctx, String> = Fx::pending(|c: Ctx| Fx::value(c.b.clone()));
    let fx2 = fx.update_context("hello".to_owned());
    assert_eq!(fx2.provide(ctx).eval(), "hello");
}

#[test]
fn update_context_replaces_part_of_context() {
    let ctx = Ctx {
        a: 7,
        b: "init".to_owned(),
    };
    let a = Fx::func(|u: i32| u * 2).lift();
    let fx = a
        .clone()
        .map_m(|n| a.update_context(10i32).map(move |m| (n, m)));
    let v = fx.provide(ctx).eval();
    assert_eq!(v, (14, 20))
}
