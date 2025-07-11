use abilities_macro::abilities;
use fx::{Abilities, Ability, Fx, Has, Pair};

abilities! {
  pub trait Foo {
    fn length(line: String) -> usize;
  }

  trait Bar {
    fn increment(n: usize) -> usize;
    fn decrement(n: usize) -> usize;
  }
}

abilities! {
  trait Baz {
    fn len_add(ctx: String, n: u8) -> usize;
  }
}

#[test]
fn check_types() {
    let _: Foo = Foo;
    let _: Bar = Bar;
}

#[test]
fn ability_from_closure() {
    let _: Box<dyn Ability<'_, String, (), usize>> =
        Box::new(Foo::length_ability(|line: String| Fx::value(line.len())));
    let _: Box<dyn Ability<'_, String, (), usize>> = Box::new(|line: String| Fx::value(line.len()));
}

#[test]
fn check_type_of_ability_requests() {
    fn check_type<'f, P>(_: Fx<'f, P, usize>)
    where
        P: Pair<Box<dyn Ability<'f, String, (), usize> + 'f>, ()>,
    {
    }
    check_type(Abilities::request::<(_, _), _>("hello".to_owned()));
    check_type(Foo::length::<(_, _), _>("Hello".to_owned()));
}

#[test]
fn ctx_ability_from_closure() {
    let _: Box<dyn Ability<'_, u8, String, usize>> = Box::new(Baz::len_add_ability(|n: u8| {
        Fx::func(move |s: String| s.len() + (n as usize))
    }));
    let _: Box<dyn Ability<'_, u8, String, usize>> =
        Box::new(|n: u8| Fx::func(move |s: String| s.len() + (n as usize)));
}

#[test]
fn ctx_check_type_of_ability_requests() {
    fn check_type<'f, P>(_: Fx<'f, P, usize>)
    where
        P: Pair<Box<dyn Ability<'f, u8, String, usize> + 'f>, String>,
    {
    }
    check_type(Abilities::request::<(_, _), _>(10u8));
    check_type(Baz::len_add::<(_, _), _>(10u8));
}
