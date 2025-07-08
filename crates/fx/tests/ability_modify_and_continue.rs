// Integration test migrated from core/tests/ability_test.rs: modify_and_continue_delimited_continuation
use fx::{Ability, Fx};

#[derive(Clone)]
struct AS {}
impl<'f> Ability<'f, i32, (), i32> for AS {
    fn apply(&self, i: i32) -> Fx<'f, (), i32> {
        Fx::value(i + 10)
    }
}

#[test]
fn modify_and_continue_delimited_continuation() {
    // simulate Ability::request
    let v = AS {}.apply(5).map(|n| n * 2).eval();
    // Expected: (5 (ability input) + 10 (handler)) * 2 (continuation) = 30
    assert_eq!(v, 30)
}
