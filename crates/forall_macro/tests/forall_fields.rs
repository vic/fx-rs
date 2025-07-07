//! Integration tests for the forall_fields macro

use forall_macro::forall_fields;

#[derive(Debug, PartialEq)]
pub struct TestStruct {
    pub a: i32,
    pub b: bool,
    pub c: &'static str,
}

// Example law: all fields must satisfy a predicate
fn is_default<T: Default + PartialEq>(x: &T) -> bool {
    x == &T::default()
}

#[test]
fn test_forall_fields_macro_applies_predicate() {
    let s = TestStruct {
        a: 0,
        b: false,
        c: "",
    };
    // This should expand to: is_default(&s.a) && is_default(&s.b) && is_default(&s.c)
    let all_default = forall_fields!(TestStruct, |field: &_| is_default(field));
    assert!(all_default(&s));
}

#[test]
fn test_forall_fields_macro_fails_on_non_default() {
    let s = TestStruct {
        a: 1,
        b: false,
        c: "",
    };
    let all_default = forall_fields!(TestStruct, |field: &_| is_default(field));
    assert!(!all_default(&s));
}
