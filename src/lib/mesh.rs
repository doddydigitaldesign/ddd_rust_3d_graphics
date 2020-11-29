use crate::Point;

use super::{
    traits::{structifyable::Structifyable, vectorizable::Vectorizable},
    triangle::Triangle,
    types::vector::{Vec3, Vec4},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Mesh<T> {
    pub items: T,
}

impl<T> Mesh<T>
where
    T: Copy,
{
    pub fn new(items: T) -> Mesh<T> {
        Mesh { items }
    }
}

impl Vectorizable<[Vec3<Vec3<f64>>; 12]> for Mesh<[Triangle; 12]> {
    fn to_vector(&self) -> [Vec3<Vec3<f64>>; 12] {
        let mut copied_vector = [[[0f64; 3]; 3]; 12];
        for item_index in 0..self.items.len() {
            let item_value = self.items[item_index];
            copied_vector[item_index] = item_value.to_vector();
        }

        copied_vector
    }
}

impl Structifyable<[Vec4<Vec4<f64>>; 12]> for Mesh<[Triangle; 12]> {
    fn from_vector(vector: &[Vec4<Vec4<f64>>; 12]) -> Self {
        let mut tmp: [Triangle; 12] = [Triangle::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 0.0),
        ); 12];
        for item in 0..vector.len() {
            tmp[item] = Triangle::from_vector(&vector[item]);
        }
        Mesh { items: tmp }
    }
}
