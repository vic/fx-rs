use crate::{
    core::{
        has_put::{Has, Put},
        lens::Lens,
        pair::Pair,
        state::State,
    },
    kernel::fx::Fx,
};

#[test]
fn test_forall_combinator() {
    let fx: Fx<'static, i32, i32> = State::get();
    let fx2 = fx.forall(|fx| fx.map(|n| n + 1));
    let result = fx2.provide(41).eval();
    assert_eq!(result, 42);
}

#[derive(Clone, Debug, PartialEq)]
struct S2 {
    x: i32,
    y: bool,
}
impl Pair<i32, bool> for S2 {
    fn fst(self) -> i32 {
        self.x
    }
    fn snd(self) -> bool {
        self.y
    }
}
impl From<(i32, bool)> for S2 {
    fn from((x, y): (i32, bool)) -> Self {
        S2 { x, y }
    }
}
impl Into<(i32, bool)> for S2 {
    fn into(self) -> (i32, bool) {
        (self.x, self.y)
    }
}
impl Has<i32> for S2 {
    fn get(self) -> i32 {
        self.x
    }
}
impl Has<bool> for S2 {
    fn get(self) -> bool {
        self.y
    }
}
impl Put<i32> for S2 {
    fn put(mut self, value: i32) -> Self {
        self.x = value;
        self
    }
}
impl Put<bool> for S2 {
    fn put(mut self, value: bool) -> Self {
        self.y = value;
        self
    }
}

#[test]
fn test_forall_pair_state_with_lens() {
    let lens = Lens::<S2, i32>::new();
    let fx: Fx<'static, S2, S2> = Fx::immediate(S2 { x: 21, y: true }, S2 { x: 21, y: true })
        .via(lens.zoom_in(|_| State::<i32>::get().map_m(|n| State::set(n * 2).then(State::get()))))
        .then(State::get());
    let result = fx.provide(S2 { x: 21, y: true }).eval();
    assert_eq!(result, S2 { x: 42, y: true });
}

#[test]
fn test_forall_composed() {
    let fx: Fx<'static, i32, i32> = State::get();
    let fx2 = fx.forall(|fx| fx.map(|n| n + 1));
    let fx3 = fx2.forall(|fx| fx.map(|n| n * 2));
    let result = fx3.provide(10).eval();
    assert_eq!(result, 22);
}
