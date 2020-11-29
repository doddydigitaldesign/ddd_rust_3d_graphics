use crate::scalable::Scalable;

use super::{
    traits::{structifyable::Structifyable, vectorizable::Vectorizable},
    types::vector::{Vec3, Vec4},
};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }
}

impl Vectorizable<Vec3<f64>> for Point {
    fn to_vector(&self) -> Vec3<f64> {
        [self.x, self.y, self.z]
    }
}

impl Structifyable<Vec4<f64>> for Point {
    fn from_vector(vector: &Vec4<f64>) -> Point {
        Point {
            x: vector[0],
            y: vector[1],
            z: vector[2],
        }
    }
}

impl Scalable for Point {
    fn scale(&mut self, factor: f64) -> Self {
        self.x = self.x * factor;
        self.y = self.y * factor;
        self.z = self.z * factor;

        *self
    }
}
