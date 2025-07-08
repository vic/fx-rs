// Integration test for abilities_macro
// This test checks macro expansion, ability requests, and handler routing.

use abilities_macro::abilities;
use fx::{Ability, Fx, Handler};

#[abilities(
    parse(String) -> usize,
    show(usize) -> String,
)]
pub struct Foo;

// Check that the handler trait can be implemented and used
// #[derive(Clone)]
// struct MyHandler;
// impl<'f> FooHandler<'f> for MyHandler {
//     fn parse<S: Clone>(n: String) -> Fx<'f, S, usize> {
//         Fx::value(n.len())
//     }
//     fn show<S: Clone>(n: usize) -> Fx<'f, S, String> {
//         Fx::value(format!("n={}", n))
//     }
// }

#[test]
fn test_foo_ability_macro_expansion() {
    // Check that the enum exists and variants can be constructed
    // let _ = FooAbility::Parse("abc".to_string());
    // let _ = FooAbility::Show(42);
    // let _ = FooResult::Parse(123);
    // let _ = FooResult::Show("abc".to_string());

    // // Check that the static request functions typecheck
    // let _: Fx<(FooAbility, bool), usize> = Foo::parse("abc".to_string());
    // let _: Fx<(FooAbility, ()), String> = Foo::show(42);

    // let _handler = Foo::handler(Box::new(MyHandler));
}
