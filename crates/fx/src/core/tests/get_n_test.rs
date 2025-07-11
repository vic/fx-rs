use crate::core::has_put::Has;
use crate::core::state::State;

// Common Ctx and Has impls for primitive tuple tests
#[derive(Clone)]
struct Ctx2 {
    a: u8,
    b: u16,
}
impl Has<u8> for Ctx2 {
    fn get(self) -> u8 {
        self.a
    }
}
impl Has<u16> for Ctx2 {
    fn get(self) -> u16 {
        self.b
    }
}

#[derive(Clone)]
struct Ctx3 {
    a: u8,
    b: u16,
    c: u32,
}
impl Has<u8> for Ctx3 {
    fn get(self) -> u8 {
        self.a
    }
}
impl Has<u16> for Ctx3 {
    fn get(self) -> u16 {
        self.b
    }
}
impl Has<u32> for Ctx3 {
    fn get(self) -> u32 {
        self.c
    }
}

#[derive(Clone)]
struct Ctx4 {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
}
impl Has<u8> for Ctx4 {
    fn get(self) -> u8 {
        self.a
    }
}
impl Has<u16> for Ctx4 {
    fn get(self) -> u16 {
        self.b
    }
}
impl Has<u32> for Ctx4 {
    fn get(self) -> u32 {
        self.c
    }
}
impl Has<u64> for Ctx4 {
    fn get(self) -> u64 {
        self.d
    }
}

#[derive(Clone)]
struct Ctx5 {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
    e: usize,
}
impl Has<u8> for Ctx5 {
    fn get(self) -> u8 {
        self.a
    }
}
impl Has<u16> for Ctx5 {
    fn get(self) -> u16 {
        self.b
    }
}
impl Has<u32> for Ctx5 {
    fn get(self) -> u32 {
        self.c
    }
}
impl Has<u64> for Ctx5 {
    fn get(self) -> u64 {
        self.d
    }
}
impl Has<usize> for Ctx5 {
    fn get(self) -> usize {
        self.e
    }
}

#[derive(Clone)]
struct Ctx6 {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
    e: usize,
    f: i8,
}
impl Has<u8> for Ctx6 {
    fn get(self) -> u8 {
        self.a
    }
}
impl Has<u16> for Ctx6 {
    fn get(self) -> u16 {
        self.b
    }
}
impl Has<u32> for Ctx6 {
    fn get(self) -> u32 {
        self.c
    }
}
impl Has<u64> for Ctx6 {
    fn get(self) -> u64 {
        self.d
    }
}
impl Has<usize> for Ctx6 {
    fn get(self) -> usize {
        self.e
    }
}
impl Has<i8> for Ctx6 {
    fn get(self) -> i8 {
        self.f
    }
}

#[derive(Clone)]
struct Ctx7 {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
    e: usize,
    f: i8,
    g: i16,
}
impl Has<u8> for Ctx7 {
    fn get(self) -> u8 {
        self.a
    }
}
impl Has<u16> for Ctx7 {
    fn get(self) -> u16 {
        self.b
    }
}
impl Has<u32> for Ctx7 {
    fn get(self) -> u32 {
        self.c
    }
}
impl Has<u64> for Ctx7 {
    fn get(self) -> u64 {
        self.d
    }
}
impl Has<usize> for Ctx7 {
    fn get(self) -> usize {
        self.e
    }
}
impl Has<i8> for Ctx7 {
    fn get(self) -> i8 {
        self.f
    }
}
impl Has<i16> for Ctx7 {
    fn get(self) -> i16 {
        self.g
    }
}

#[derive(Clone)]
struct Ctx8 {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
    e: usize,
    f: i8,
    g: i16,
    h: i32,
}
impl Has<u8> for Ctx8 {
    fn get(self) -> u8 {
        self.a
    }
}
impl Has<u16> for Ctx8 {
    fn get(self) -> u16 {
        self.b
    }
}
impl Has<u32> for Ctx8 {
    fn get(self) -> u32 {
        self.c
    }
}
impl Has<u64> for Ctx8 {
    fn get(self) -> u64 {
        self.d
    }
}
impl Has<usize> for Ctx8 {
    fn get(self) -> usize {
        self.e
    }
}
impl Has<i8> for Ctx8 {
    fn get(self) -> i8 {
        self.f
    }
}
impl Has<i16> for Ctx8 {
    fn get(self) -> i16 {
        self.g
    }
}
impl Has<i32> for Ctx8 {
    fn get(self) -> i32 {
        self.h
    }
}

#[derive(Clone)]
struct Ctx9 {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
    e: usize,
    f: i8,
    g: i16,
    h: i32,
    i: i64,
}
impl Has<u8> for Ctx9 {
    fn get(self) -> u8 {
        self.a
    }
}
impl Has<u16> for Ctx9 {
    fn get(self) -> u16 {
        self.b
    }
}
impl Has<u32> for Ctx9 {
    fn get(self) -> u32 {
        self.c
    }
}
impl Has<u64> for Ctx9 {
    fn get(self) -> u64 {
        self.d
    }
}
impl Has<usize> for Ctx9 {
    fn get(self) -> usize {
        self.e
    }
}
impl Has<i8> for Ctx9 {
    fn get(self) -> i8 {
        self.f
    }
}
impl Has<i16> for Ctx9 {
    fn get(self) -> i16 {
        self.g
    }
}
impl Has<i32> for Ctx9 {
    fn get(self) -> i32 {
        self.h
    }
}
impl Has<i64> for Ctx9 {
    fn get(self) -> i64 {
        self.i
    }
}

#[test]
fn get2_extracts_tuple_from_context() {
    #[derive(Clone, Debug, PartialEq)]
    struct Foo(&'static str);
    #[derive(Clone, Debug, PartialEq)]
    struct Bar(u8);
    #[derive(Clone)]
    struct Ctx {
        foo: Foo,
        bar: Bar,
    }
    impl Has<Foo> for Ctx {
        fn get(self) -> Foo {
            self.foo
        }
    }
    impl Has<Bar> for Ctx {
        fn get(self) -> Bar {
            self.bar
        }
    }
    let ctx = Ctx {
        foo: Foo("hi"),
        bar: Bar(42),
    };
    let fx = State::<Ctx>::get2::<Foo, Bar>();
    let (foo, bar) = fx.provide(ctx.clone()).eval();
    assert_eq!(foo, Foo("hi"));
    assert_eq!(bar, Bar(42));
}

#[test]
fn get2_works_with_primitives() {
    let ctx = Ctx2 { a: 7, b: 99 };
    let fx = State::<Ctx2>::get2::<u8, u16>();
    let (a, b) = fx.provide(ctx.clone()).eval();
    assert_eq!(a, 7);
    assert_eq!(b, 99);
}

#[test]
fn get3_extracts_tuple() {
    let ctx = Ctx3 { a: 1, b: 2, c: 3 };
    let fx = State::<Ctx3>::get3::<u8, u16, u32>();
    let (a, b, c) = fx.provide(ctx.clone()).eval();
    assert_eq!((a, b, c), (1, 2, 3));
}

#[test]
fn get4_extracts_tuple() {
    let ctx = Ctx4 {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
    };
    let fx = State::<Ctx4>::get4::<u8, u16, u32, u64>();
    let (a, b, c, d) = fx.provide(ctx.clone()).eval();
    assert_eq!((a, b, c, d), (1, 2, 3, 4));
}

#[test]
fn get5_extracts_tuple() {
    let ctx = Ctx5 {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: 5,
    };
    let fx = State::<Ctx5>::get5::<u8, u16, u32, u64, usize>();
    let (a, b, c, d, e) = fx.provide(ctx.clone()).eval();
    assert_eq!((a, b, c, d, e), (1, 2, 3, 4, 5));
}

#[test]
fn get6_extracts_tuple() {
    let ctx = Ctx6 {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: 5,
        f: 6,
    };
    let fx = State::<Ctx6>::get6::<u8, u16, u32, u64, usize, i8>();
    let (a, b, c, d, e, f) = fx.provide(ctx.clone()).eval();
    assert_eq!((a, b, c, d, e, f), (1, 2, 3, 4, 5, 6));
}

#[test]
fn get7_extracts_tuple() {
    let ctx = Ctx7 {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: 5,
        f: 6,
        g: 7,
    };
    let fx = State::<Ctx7>::get7::<u8, u16, u32, u64, usize, i8, i16>();
    let (a, b, c, d, e, f, g) = fx.provide(ctx.clone()).eval();
    assert_eq!((a, b, c, d, e, f, g), (1, 2, 3, 4, 5, 6, 7));
}

#[test]
fn get8_extracts_tuple() {
    let ctx = Ctx8 {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: 5,
        f: 6,
        g: 7,
        h: 8,
    };
    let fx = State::<Ctx8>::get8::<u8, u16, u32, u64, usize, i8, i16, i32>();
    let (a, b, c, d, e, f, g, h) = fx.provide(ctx.clone()).eval();
    assert_eq!((a, b, c, d, e, f, g, h), (1, 2, 3, 4, 5, 6, 7, 8));
}

#[test]
fn get9_extracts_tuple() {
    let ctx = Ctx9 {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: 5,
        f: 6,
        g: 7,
        h: 8,
        i: 9,
    };
    let fx = State::<Ctx9>::get9::<u8, u16, u32, u64, usize, i8, i16, i32, i64>();
    let (a, b, c, d, e, f, g, h, i) = fx.provide(ctx.clone()).eval();
    assert_eq!((a, b, c, d, e, f, g, h, i), (1, 2, 3, 4, 5, 6, 7, 8, 9));
}
