use crate::kernel::fx::Fx;

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
