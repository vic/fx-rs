use crate::{Fx, State};
use crate::{Has, Lens, Put};

#[derive(Clone, Debug, PartialEq)]
struct Ctx {
    a: u32,
    b: &'static str,
}

impl Has<u32> for Ctx {
    fn get(&self) -> &u32 {
        &self.a
    }
}
impl Put<u32> for Ctx {
    fn put(mut self, value: u32) -> Self {
        self.a = value;
        self
    }
}

#[cfg(test)]
mod lens_tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    struct ST {
        a: i32,
        b: String,
    }
    impl ST {
        fn b<'f>() -> Lens<'f, ST, String> {
            Lens::new(|s: ST| s.b, |s, b| ST { b, ..s })
        }
        fn a<'f>() -> Lens<'f, ST, i32> {
            Lens::new(|s: ST| s.a, |s, a| ST { a, ..s })
        }
    }

    #[test]
    fn test_focus_out() {
        let e: Fx<String, ()> = State::set("hello".to_string()).then(Fx::value(()));
        let e: Fx<ST, ()> = e.via(ST::b().zoom_out());
        let e = e.then(State::get());
        let result = e
            .provide(ST {
                a: 0,
                b: "world".to_string(),
            })
            .eval();
        assert_eq!(
            result,
            ST {
                a: 0,
                b: "hello".to_string()
            }
        );
    }

    #[test]
    fn test_focus_in() {
        let e: Fx<ST, ()> = Fx::immediate(
            ST {
                a: 42,
                b: "hello".to_string(),
            },
            (),
        );
        let e: Fx<ST, ()> = e.via(ST::b().zoom_in(|()| State::set("bye".to_string()).map(|_| ())));
        let e = e.then(State::get());
        let result = e
            .provide(ST {
                a: 0,
                b: "bad".to_string(),
            })
            .eval();
        assert_eq!(
            result,
            ST {
                a: 42,
                b: "bye".to_string()
            }
        );
    }

    #[test]
    fn test_focus_in_and_out() {
        let inner: Fx<i32, ()> = Fx::immediate(10, ());
        let outer: Fx<ST, ()> = inner
            .via(ST::a().zoom_out())
            .then(State::map(|s: ST| ST { a: s.a * 2, ..s }))
            .then(Fx::value(()));
        let back: Fx<ST, i32> = outer.via(ST::a().zoom_in(|_| State::<i32>::map(|n| n + 10)));
        let e = back.then(State::get());
        let result = e
            .provide(ST {
                a: 0,
                b: "hello".to_owned(),
            })
            .eval();
        assert_eq!(
            result,
            ST {
                a: 30,
                b: "hello".to_owned()
            }
        );
    }
}

#[test]
fn lens_from_has_put_get_set() {
    let ctx = Ctx { a: 1, b: "hi" };
    let lens = Lens::<'_, Ctx, u32>::from_has_put();
    // Test get
    let got = lens.get(ctx.clone());
    assert_eq!(got, 1);
    // Test set
    let set = lens.set(ctx.clone(), 42);
    assert_eq!(set, Ctx { a: 42, b: "hi" });
}
