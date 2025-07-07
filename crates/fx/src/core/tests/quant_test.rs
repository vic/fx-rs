use crate::{Fx, State};
use crate::core::quant::Quantify;

#[test]
fn test_quantify_combinator() {
    // Simple stateful computation
    let fx: Fx<'static, i32, i32> = State::get();
    // Quantify: wrap with a combinator that adds 1 to the result
    let fx2 = fx.quantify(|fx| fx.map(|n| n + 1));
    let result = fx2.provide(41).eval();
    assert_eq!(result, 42);
}
