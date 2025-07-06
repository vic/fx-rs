pub trait Product<A, B>
where
    Self: From<(A, B)> + Into<(A, B)> + Sized,
{
}

impl<A, B> Product<A, B> for (A, B) {}
