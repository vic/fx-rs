use crate::core::has_put::{Has, Put};
use crate::core::lens::Lens;
use crate::core::state::State;
use crate::kernel::fx::Fx;

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

#[derive(Clone, Debug, PartialEq)]
struct ST {
    a: i32,
    b: String,
}

impl Has<i32> for ST {
    fn get(&self) -> &i32 {
        &self.a
    }
}
impl Put<i32> for ST {
    fn put(mut self, value: i32) -> Self {
        self.a = value;
        self
    }
}
impl Has<String> for ST {
    fn get(&self) -> &String {
        &self.b
    }
}
impl Put<String> for ST {
    fn put(mut self, value: String) -> Self {
        self.b = value;
        self
    }
}
impl ST {
    fn a<'f>() -> Lens<'f, ST, i32> {
        Lens::new()
    }
    fn b<'f>() -> Lens<'f, ST, String> {
        Lens::new()
    }
}

#[test]
fn from_has_put_lens() {
    let lens: Lens<Ctx, u32> = Lens::new();
    let ctx = Ctx { a: 1, b: "hi" };
    assert_eq!(lens.get(ctx.clone()), 1);
    let updated = lens.set(ctx, 42);
    assert_eq!(updated, Ctx { a: 42, b: "hi" });
}

#[test]
fn test_focus_out() {
    let e: Fx<String, ()> = State::set("hello".to_string()).then(Fx::value(()));
    let e: Fx<ST, ()> = e.via(ST::b().zoom_out());
    let e: Fx<ST, ST> = e.then(State::get());
    let result: ST = e
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
    let e: Fx<ST, ST> = e.then(State::get());
    let result: ST = e
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
    let e: Fx<ST, ST> = back.then(State::get());
    let result: ST = e
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

#[test]
fn prepend_composes_lenses() {
    // This test demonstrates the intended usage of Lens::prepend (currently commented out).
    // We use ST as Outer, and Lens<ST, i32> and Lens<i32, i32> as inner/outer lenses for demonstration.
    #[derive(Clone, Debug, PartialEq)]
    struct Outer {
        st: ST,
    }
    impl Has<ST> for Outer {
        fn get(&self) -> &ST {
            &self.st
        }
    }
    impl Put<ST> for Outer {
        fn put(mut self, value: ST) -> Self {
            self.st = value;
            self
        }
    }
    let outer = Outer {
        st: ST {
            a: 5,
            b: "x".to_string(),
        },
    };
    let st_lens = Lens::<Outer, ST>::new();
    let a_lens = Lens::<ST, i32>::new();
    let composed = a_lens.prepend(st_lens);
    assert_eq!(composed.get(outer.clone()), 5);
    let updated = composed.set(outer, 99);
    assert_eq!(updated.st.a, 99);
}

#[test]
fn append_composes_lenses() {
    // This test demonstrates the intended usage of Lens::append.
    // We use ST as Outer, and Lens<ST, i32> and Lens<i32, i32> as inner/right lenses for demonstration.
    #[derive(Clone, Debug, PartialEq)]
    struct Outer {
        st: ST,
    }
    impl Has<ST> for Outer {
        fn get(&self) -> &ST {
            &self.st
        }
    }
    impl Put<ST> for Outer {
        fn put(mut self, value: ST) -> Self {
            self.st = value;
            self
        }
    }
    let outer = Outer {
        st: ST {
            a: 5,
            b: "x".to_string(),
        },
    };
    let st_lens = Lens::<Outer, ST>::new();
    let a_lens = Lens::<ST, i32>::new();
    let composed = st_lens.append(a_lens);
    assert_eq!(composed.get(outer.clone()), 5);
    let updated = composed.set(outer, 123);
    assert_eq!(updated.st.a, 123);
}

#[test]
fn left_lens_accesses_left_of_tuple() {
    let pair: (i32, &str) = (10, "hi");
    let left = Lens::<'_, (i32, &str), i32>::left::<&str>();
    assert_eq!(left.get(pair.clone()), 10);
    let updated = left.set(pair, 42);
    assert_eq!(updated.0, 42);
}

#[test]
fn right_lens_accesses_right_of_tuple() {
    let pair: (i32, &str) = (10, "hi");
    let right = Lens::<'_, (i32, &str), &str>::right::<i32>();
    assert_eq!(right.get(pair.clone()), "hi");
    let updated = right.set(pair, "bye");
    assert_eq!(updated.1, "bye");
}
