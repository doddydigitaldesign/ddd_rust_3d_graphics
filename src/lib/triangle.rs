use piston_window::types::Polygons;

use crate::{drawable::Drawable, into_polygons::IntoPolygons, scalable::Scalable};

use super::{
    point::Point,
    traits::{structifyable::Structifyable, vectorizable::Vectorizable},
    types::vector::Vec3,
    types::vector::Vec4,
};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Triangle {
        Triangle { a, b, c }
    }
}

impl Vectorizable<Vec3<Vec3<f64>>> for Triangle {
    fn to_vector(&self) -> Vec3<Vec3<f64>> {
        [self.a.to_vector(), self.b.to_vector(), self.c.to_vector()]
    }
}

impl Structifyable<Vec4<Vec4<f64>>> for Triangle {
    fn from_vector(vector: &Vec4<Vec4<f64>>) -> Self {
        Triangle {
            a: Point::from_vector(&vector[0]),
            b: Point::from_vector(&vector[1]),
            c: Point::from_vector(&vector[2]),
        }
    }
}

impl IntoPolygons<[[f64; 2]; 3]> for Triangle {
    fn into_polygon(&self) -> [[f64; 2]; 3] {
        let a = self.a;
        let b = self.b;
        let c = self.c;

        let polygon = [[a.x, a.y], [b.x, b.y], [c.x, c.y]];
        polygon
    }
}

impl Scalable for Triangle {
    fn scale(&self, factor: f64) -> Triangle {
        Triangle {
            a: self.a.scale(factor),
            b: self.b.scale(factor),
            c: self.c.scale(factor),
        }
    }
}
