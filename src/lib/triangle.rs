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

// impl Structifyable<Vec3<Vec3<f64>>> for Triangle {
//     fn from_vector(vector: &Vec3<Vec3<f64>>) -> Self {
//         Triangle {
//             a: Point::from_vector(&vector[0]),
//             b: Point::from_vector(&vector[1]),
//             c: Point::from_vector(&vector[2]),
//         }
//     }
// }
impl Structifyable<Vec4<Vec4<f64>>> for Triangle {
    fn from_vector(vector: &Vec4<Vec4<f64>>) -> Self {
        Triangle {
            a: Point::from_vector(&vector[0]),
            b: Point::from_vector(&vector[1]),
            c: Point::from_vector(&vector[2]),
        }
    }
}
