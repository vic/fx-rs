use dyn_clone::{DynClone, clone_trait_object};

pub trait Has<T>
where
    Self: DynClone,
{
    fn get<'f>(&'f self) -> &'f T;
}

impl<T: Clone> Has<T> for T {
    fn get<'f>(&'f self) -> &'f T {
        self
    }
}

pub trait Put<T> {
    fn put(self, value: T) -> Self;
}

impl<T: Clone> Put<T> for T {
    fn put(self, value: T) -> Self {
        value
    }
}

pub trait HasPut<T>
where
    Self: Has<T> + Put<T>,
{
}

impl<I, O> HasPut<I> for O where O: Has<I> + Put<I> {}
clone_trait_object!(<T> Has<T>);
