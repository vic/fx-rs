pub trait Has<T> {
    fn get(&self) -> &T;
}

impl<T: Clone> Has<T> for T {
    fn get(&self) -> &T {
        self
    }
}
