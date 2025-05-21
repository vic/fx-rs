#[derive(Copy, Clone)]
pub struct And<A, B>(A, B);

impl<A, B> And<A, B> {
    pub fn new(a: A, b: B) -> Self {
        And(a, b)
    }
    pub fn swap(self) -> And<B, A> {
        And(self.1, self.0)
    }
    pub fn left(self) -> A {
        self.0
    }
    pub fn right(self) -> B {
        self.1
    }
    pub fn tuple(self) -> (A, B) {
        (self.0, self.1)
    }
}
