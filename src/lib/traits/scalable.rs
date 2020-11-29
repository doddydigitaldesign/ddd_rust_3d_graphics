pub trait Scalable {
    fn scale(&mut self, factor: f64) -> Self;
}
