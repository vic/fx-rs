use super::{Lens, Has, Put};

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
