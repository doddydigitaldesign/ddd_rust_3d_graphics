pub trait Structifyable<T> {
    fn from_vector(vector: &T) -> Self;
}
