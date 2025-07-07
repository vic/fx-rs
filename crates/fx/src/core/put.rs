pub trait Put<T> {
    fn put(self, value: T) -> Self;
}

impl<T: Clone> Put<T> for T {
    fn put(self, value: T) -> Self {
        value
    }
}
