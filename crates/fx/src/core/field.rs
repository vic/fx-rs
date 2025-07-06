pub trait Field<T> {
    fn field(&self) -> &T;
}

impl<T: Clone> Field<T> for T {
    fn field(&self) -> &T {
        self
    }
}
