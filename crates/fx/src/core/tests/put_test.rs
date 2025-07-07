use super::Put;

#[test]
fn put_identity_for_leaf() {
    let x = 5;
    let y = 10;
    let z = x.put(y);
    assert_eq!(z, 10);
}

#[derive(Clone, Debug, PartialEq)]
struct Ctx {
    a: u32,
    b: &'static str,
}

impl Put<u32> for Ctx {
    fn put(mut self, value: u32) -> Self {
        self.a = value;
        self
    }
}

#[test]
fn put_field_in_struct() {
    let ctx = Ctx { a: 1, b: "hi" };
    let ctx2 = ctx.clone().put(42);
    assert_eq!(ctx2, Ctx { a: 42, b: "hi" });
}
