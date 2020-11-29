pub trait Vectorizable<T> {
    fn to_vector(&self) -> T;
}
