pub trait Merge<T>
where
    Self: Default,
{
    /// Merge with other object
    #[must_use]
    fn merge(self, other: T) -> Self;

    fn merge_with_default(other: T) -> Self {
        Self::merge(Self::default(), other)
    }
}
