use crate::{kernel::fx::Fx, Put};

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
    let ctx = Ctx { a: 0, b: "init".to_owned() };
    let fx: Fx<Ctx, i32> = Fx::pending(|c: Ctx| Fx::value(c.a));
    let fx2 = fx.provide_has(42);
    assert_eq!(fx2.provide(ctx.clone()).eval(), 42);
}

#[test]
fn provide_has_sets_string_field() {
    let ctx = Ctx { a: 7, b: "init".to_owned() };
    let fx: Fx<Ctx, String> = Fx::pending(|c: Ctx| Fx::value(c.b.clone()));
    let fx2 = fx.provide_has("hello".to_owned());
    assert_eq!(fx2.provide(ctx.clone()).eval(), "hello");
}
