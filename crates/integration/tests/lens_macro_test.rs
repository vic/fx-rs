use fx::{Fx, State};
use fx_lens::Lens;

#[derive(Clone, Debug, PartialEq, Lens)]
struct ST {
    a: i32,
    b: String,
}

impl fx::Has<i32> for ST {
    fn get(&self) -> &i32 {
        &self.a
    }
}
impl fx::Put<i32> for ST {
    fn put(mut self, value: i32) -> Self {
        self.a = value;
        self
    }
}
impl fx::Has<String> for ST {
    fn get(&self) -> &String {
        &self.b
    }
}
impl fx::Put<String> for ST {
    fn put(mut self, value: String) -> Self {
        self.b = value;
        self
    }
}

#[test]
fn integration_focus_out() {
    let e: Fx<String, ()> = State::set("hello".to_string()).then(Fx::value(()));
    let e: Fx<ST, ()> = e.via(ST::lens_b().zoom_out());
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
fn integration_focus_in() {
    let e: Fx<ST, ()> = Fx::immediate(
        ST {
            a: 42,
            b: "hello".to_string(),
        },
        (),
    );
    let e: Fx<ST, ()> = e.via(ST::lens_b().zoom_in(|()| State::set("bye".to_string()).map(|_| ())));
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
fn integration_focus_in_and_out() {
    let inner: Fx<i32, ()> = Fx::immediate(10, ());
    let outer: Fx<ST, ()> = inner
        .via(ST::lens_a().zoom_out())
        .then(State::map(|s: ST| ST { a: s.a * 2, ..s }))
        .then(Fx::value(()));
    let back: Fx<ST, i32> = outer.via(ST::lens_a().zoom_in(|_| State::<i32>::map(|n| n + 10)));
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

// Macro-generated lens accessors now use Lens::from(())
