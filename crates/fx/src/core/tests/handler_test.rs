use crate::core::handler::Handler;
use crate::kernel::fx::Fx;

#[test]
fn handler_fn_once_works() {
    let h = |fx: Fx<'static, (), i32>| fx.map(|x| x + 1);
    let fx = Fx::value(41);
    let result = h.handle(fx).eval();
    assert_eq!(result, 42);
}

#[test]
fn handler_clone_works() {
    let h = |fx: Fx<'static, (), i32>| fx.map(|x| x * 2);
    let fx = Fx::value(21);
    let h2 = h.clone();
    let result = h2.handle(fx).eval();
    assert_eq!(result, 42);
}
