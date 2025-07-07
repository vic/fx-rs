use fx::Has;

#[test]
fn test_field_struct() {
    #[derive(Clone, Debug, PartialEq, fx_field::Field)]
    struct Ctx {
        a: u32,
        b: &'static str,
    }
    let ctx = Ctx { a: 42, b: "hello" };
    assert_eq!(*<Ctx as Has<u32>>::get(&ctx), 42u32);
    assert_eq!(*<Ctx as Has<&'static str>>::get(&ctx), "hello");
}
