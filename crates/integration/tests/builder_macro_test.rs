use builder_macro::{ContextBuilder};
use builder_types::{Absent, Present};
use fx::Has;
use fx_field::HasFields;

#[derive(HasFields, ContextBuilder, Debug, PartialEq, Clone)]
struct Foo {
    a: usize,
    b: String,
    c: bool,
}

#[test]
fn all_fields() {
    let builder = FooBuilder::empty().a(42).b("hello".to_owned()).c(true);
    let foo = builder.build();
    assert_eq!(
        foo,
        Foo {
            a: 42,
            b: "hello".to_owned(),
            c: true
        }
    );
}

#[test]
fn partial_fields() {
    let builder = FooBuilder::empty().a(1);
    // Should not be able to call build() yet (type error if uncommented)
    // let foo = builder.build();
    assert_eq!(builder.maybe_a(), Some(1));
    assert_eq!(builder.maybe_b(), None);
    assert_eq!(builder.maybe_c(), None);
}

#[test]
fn fields_any_order() {
    let builder = FooBuilder::<Absent, Absent, Absent>::empty()
        .c(true)
        .b("world".to_owned())
        .a(99);
    let foo = builder.build();
    assert_eq!(
        foo,
        Foo {
            a: 99,
            b: "world".to_owned(),
            c: true
        }
    );
}

#[test]
fn double_put_overwrites() {
    let builder = FooBuilder::<Absent, Absent, Absent>::empty()
        .a(1)
        .a(2)
        .b("x".to_owned())
        .c(false);
    let foo = builder.build();
    assert_eq!(
        foo,
        Foo {
            a: 2,
            b: "x".to_owned(),
            c: false
        }
    );
}

#[test]
fn double_put_all_fields() {
    let builder = FooBuilder::<Absent, Absent, Absent>::empty()
        .a(1)
        .a(2)
        .b("x".to_owned())
        .b("y".to_owned())
        .c(false)
        .c(true);
    let foo = builder.build();
    assert_eq!(
        foo,
        Foo {
            a: 2,
            b: "y".to_owned(),
            c: true
        }
    );
}

#[test]
fn all_fields_none() {
    let builder = FooBuilder::<Absent, Absent, Absent>::empty();
    assert_eq!(builder.maybe_a(), None);
    assert_eq!(builder.maybe_b(), None);
    assert_eq!(builder.maybe_c(), None);
}

#[test]
fn all_fields_default() {
    let builder = FooBuilder::<Absent, Absent, Absent>::empty()
        .a(Default::default())
        .b(Default::default())
        .c(Default::default());
    let foo = builder.build();
    assert_eq!(
        foo,
        Foo {
            a: 0,
            b: String::new(),
            c: false
        }
    );
}

#[test]
fn has_trait_impl_only_when_field_present() {
    // Builder with field 'a' present
    let builder_start = FooBuilder::<Absent, Absent, Absent>::empty();
    let builder = builder_start.a(1); // Now FooBuilder<Present, Absent, Absent>
    fn assert_has_a<T: Has<usize>>(_t: &T) {}
    assert_has_a(&builder); // Should compile

    // Builder with field 'a' absent
    let builder_absent = FooBuilder::<Absent, Absent, Absent>::empty();
    // The following line should fail to compile if uncommented:
    // assert_has_a(&builder_absent);
}
