use crate::core::arrow::{Arrow, Arrows};

#[test]
fn arrow_new_direct_usage() {
    let add = Arrows::new(|x: i32| x * 3);
    let result = add.apply(7);
    assert_eq!(result, 21);
}

#[derive(Clone)]
struct AddOne;
impl<'f> Arrow<'f, u32, u32> for AddOne {
    fn apply(self, i: u32) -> u32 {
        i + 1
    }
}

#[derive(Clone)]
struct StrLen;
impl<'f> Arrow<'f, &'f str, usize> for StrLen {
    fn apply(self, i: &'f str) -> usize {
        i.len()
    }
}

#[test]
fn arrow_request_and_apply() {
    let arr = AddOne;
    let fx = Arrows::request::<u32, u32, AddOne>(41u32);
    let result = fx.provide(arr).eval();
    assert_eq!(result, 42);
}

#[test]
fn arrow_handler_composition() {
    let arr = StrLen;
    let fx = Arrows::request::<&str, usize, StrLen>("hello");
    let result = fx.provide(arr).eval();
    assert_eq!(result, 5);
}

#[derive(Clone)]
struct AddOneU32;
impl<'f> Arrow<'f, u32, u32> for AddOneU32 {
    fn apply(self, i: u32) -> u32 {
        i + 1
    }
}

#[test]
fn arrow_adapt_maps_input_and_output() {
    let arr = AddOneU32;
    let arr2 = arr.adapt(|s: &str| s.len() as u32, |o| o * 2);
    let fx = Arrows::request::<&str, u32, _>("abc");
    let result = fx.provide(arr2).eval();
    assert_eq!(result, 8); // (len("abc") = 3) + 1 = 4, *2 = 8
}
