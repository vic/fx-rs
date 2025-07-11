use dyn_clone::{DynClone, clone_trait_object};

pub trait Has<T>
where
    Self: DynClone,
{
    fn get(self) -> T;
}

impl<T: Clone> Has<T> for T {
    fn get(self) -> T {
        self
    }
}

pub trait Put<T, U> {
    fn put(self, value: T) -> U;
}

impl<T: Clone> Put<T, T> for T {
    fn put(self, value: T) -> T {
        value
    }
}

pub trait HasPut<T, U>
where
    Self: Has<T> + Put<T, U>,
{
}

impl<I, O, U> HasPut<I, U> for O where O: Has<I> + Put<I, U> {}
clone_trait_object!(<T> Has<T>);
