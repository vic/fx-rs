use crate::core::has_put::{Has, HasPut, Put};

#[test]
fn has_and_put_for_u32() {
    let x: u32 = 7;
    assert_eq!(x.get(), 7);
    let y = x.put(42);
    assert_eq!(y, 42);
}

#[test]
fn has_and_put_for_string() {
    let s: String = "hello".to_owned();
    assert_eq!(s.clone().get(), "hello".to_owned());
    let t = s.put("world".to_owned());
    assert_eq!(t, "world");
}

#[test]
fn has_put_trait_bound_for_u32() {
    fn assert_has_put<T: HasPut<u32, T>>() {}
    assert_has_put::<u32>();
}
