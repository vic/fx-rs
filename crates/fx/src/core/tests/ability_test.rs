use crate::core::ability::{Abilities, AbilityExt};
use crate::core::handler::Handler;
use crate::core::has_put::{Has, Put};
use crate::core::state::State;
use crate::kernel::ability::Ability;
use crate::kernel::fx::Fx;
use std::usize;

fn modify_and_continue_delimited_continuation<'f, A>(ability: A)
where
    A: Ability<'f, usize, (), usize> + Clone + 'f,
{
    // work over tuple environment.
    let e = Abilities::request::<(_, _), _>(5).map(|n| n * 2);

    let h = ability.handler();
    let e = h.handle(e);
    let v = e.eval();

    // Expected: (5 (ability input) + 10 (handler)) * 2 (continuation) = 30
    assert_eq!(v, 30)
}

#[test]
fn fn_modify_and_continue() {
    let ab = Abilities::new(|n: usize| Fx::pure(n + 10));
    modify_and_continue_delimited_continuation(ab);
}

#[test]
fn boxed_modify_and_continue() {
    let ab: Box<dyn Ability<'_, usize, (), usize>> = Abilities::boxed(|n: usize| Fx::pure(n + 10));
    modify_and_continue_delimited_continuation(ab);
}

#[test]
fn struct_modify_and_continue() {
    #[derive(Clone)]
    struct A;
    impl<'f> Ability<'f, usize, (), usize> for A {
        fn apply(&self, i: usize) -> Fx<'f, (), usize> {
            Fx::pure(i + 10)
        }
    }
    modify_and_continue_delimited_continuation(A);
}

#[test]
fn ctx_map_ability() {
    let e = Abilities::request::<(_, _), _>(1u8).then(Abilities::request(2u8));
    let ab = Abilities::new(|i: u8| State::map(move |n: u8| n + i));
    let v = e.via(ab.handler()).provide(10).eval();
    assert_eq!(v, 13)
}

#[test]
fn rw_ability() {
    let read = State::<u8>::get();
    let write = State::<u8>::map(|n| n * 10);
    let e = Abilities::request::<(_, _), _>(read).then(Abilities::request(write));
    let ab = Abilities::new(|fx| fx);
    let v = e.via(ab.handler()).provide(22).eval();
    assert_eq!(v, 220)
}

#[test]
fn ability_imap_maps_input() {
    // imap should map the input type for an ability
    let ab = Abilities::new(|n: usize| Fx::pure(n + 1));
    // Map input: String -> usize
    let ab2 = ab.imap(|i: String| i.len());
    let fx = Abilities::request::<(_, _), _>("hello".to_owned());
    let fx = fx.via(ab2.handler());
    let result = fx.eval();
    assert_eq!(result, 6);
}

#[test]
fn ability_hmap_maps_handler() {
    // hmap should allow mapping the handler for an ability
    let ab = Abilities::new(|n: u32| Fx::pure(n + 1));
    // hmap: wrap the handler to double the output
    let ab2 = ab.hmap(|fx| fx.map(|n| n * 2));
    let fx = Abilities::request::<(_, _), _>(10u32);
    let fx = fx.via(ab2.handler());
    let result = fx.eval();
    assert_eq!(result, 22);
}

#[test]
fn lift_req_composes_ability_and_state() {
    #[derive(Clone)]
    struct MyAbility;
    impl<'f> Ability<'f, usize, i32, usize> for MyAbility {
        fn apply(&self, i: usize) -> Fx<'f, i32, usize> {
            Fx::pending(move |n: i32| Fx::value(i + n as usize))
        }
    }

    #[derive(Clone)]
    struct Ctx {
        ab: MyAbility,
        n: i32,
    }
    impl Has<MyAbility> for Ctx {
        fn get<'f>(&'f self) -> &'f MyAbility {
            &self.ab
        }
    }
    impl Put<MyAbility> for Ctx {
        fn put(mut self, ab: MyAbility) -> Self {
            self.ab = ab;
            self
        }
    }
    impl Has<i32> for Ctx {
        fn get<'f>(&'f self) -> &'f i32 {
            &self.n
        }
    }
    impl Put<i32> for Ctx {
        fn put(mut self, n: i32) -> Self {
            self.n = n;
            self
        }
    }

    let fx = Abilities::lift_req::<Ctx, MyAbility>(7);
    let ctx = Ctx {
        ab: MyAbility,
        n: 5,
    };
    let result = fx.provide(ctx).eval();
    assert_eq!(result, 12);
}
