//! Integration tests for the forall_fields macro

use forall_macro::ForallFields;

#[derive(Debug, PartialEq, ForallFields)]
pub struct TestStruct {
    pub a: i32,
    pub b: bool,
    pub c: &'static str,
}

fn is_default<T: Default + PartialEq + 'static>(x: &T) -> bool {
    x == &T::default()
}

#[test]
fn test_forall_fields_macro_applies_predicate() {
    let s = TestStruct {
        a: 0,
        b: false,
        c: "",
    };
    let results = s.forall_fields(|field| {
        if let Some(val) = field.downcast_ref::<i32>() {
            is_default(val)
        } else if let Some(val) = field.downcast_ref::<bool>() {
            is_default(val)
        } else if let Some(val) = field.downcast_ref::<&'static str>() {
            is_default(val)
        } else {
            false
        }
    });
    assert!(results.into_iter().all(|x| x));
}

#[test]
fn test_forall_fields_macro_fails_on_non_default() {
    let s = TestStruct {
        a: 1,
        b: false,
        c: "",
    };
    let results = s.forall_fields(|field| {
        if let Some(val) = field.downcast_ref::<i32>() {
            is_default(val)
        } else if let Some(val) = field.downcast_ref::<bool>() {
            is_default(val)
        } else if let Some(val) = field.downcast_ref::<&'static str>() {
            is_default(val)
        } else {
            false
        }
    });
    assert!(!results.into_iter().all(|x| x));
}

#[derive(Debug, PartialEq, ForallFields)]
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn test_forall_fields_non_negative() {
    let p = Point { x: 1, y: 2 };
    let results = p.forall_fields(|field| {
        if let Some(val) = field.downcast_ref::<i32>() {
            *val >= 0
        } else {
            false
        }
    });
    assert!(results.into_iter().all(|x| x));
    let p2 = Point { x: -1, y: 2 };
    let results2 = p2.forall_fields(|field| {
        if let Some(val) = field.downcast_ref::<i32>() {
            *val >= 0
        } else {
            false
        }
    });
    assert!(!results2.into_iter().all(|x| x));
}

#[derive(Debug, PartialEq, ForallFields)]
struct Mixed {
    a: i32,
    b: bool,
    c: String,
}

#[test]
fn test_forall_fields_is_default() {
    let m = Mixed {
        a: 0,
        b: false,
        c: String::new(),
    };
    let results = m.forall_fields(|field| {
        if let Some(val) = field.downcast_ref::<i32>() {
            is_default(val)
        } else if let Some(val) = field.downcast_ref::<bool>() {
            is_default(val)
        } else if let Some(val) = field.downcast_ref::<String>() {
            is_default(val)
        } else {
            false
        }
    });
    assert!(results.into_iter().all(|x| x));
    let m2 = Mixed {
        a: 1,
        b: false,
        c: String::new(),
    };
    let results2 = m2.forall_fields(|field| {
        if let Some(val) = field.downcast_ref::<i32>() {
            is_default(val)
        } else if let Some(val) = field.downcast_ref::<bool>() {
            is_default(val)
        } else if let Some(val) = field.downcast_ref::<String>() {
            is_default(val)
        } else {
            false
        }
    });
    assert!(!results2.into_iter().all(|x| x));
}

// Compile-fail/negative test stub (would require trybuild or similar framework)
// #[test]
// fn test_forall_fields_on_tuple_struct_should_fail() {
//     struct Tuple(i32, i32);
//     let _ = forall_fields!(Tuple, |field: &_| *field >= 0);
// }
