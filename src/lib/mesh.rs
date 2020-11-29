use super::{
    traits::{structifyable::Structifyable, vectorizable::Vectorizable},
    triangle::Triangle,
    types::vector::{Vec3, Vec4},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Mesh<T> {
    pub items: Vec<T>,
}

impl<T> Mesh<T>
where
    T: Copy,
{
    pub fn new(items: Vec<T>) -> Mesh<T> {
        Mesh { items }
    }
}

impl Vectorizable<Vec<Vec3<Vec3<f64>>>> for Mesh<Triangle> {
    fn to_vector(&self) -> Vec<Vec3<Vec3<f64>>> {
        let mut copied_vector: Vec<Vec3<Vec3<f64>>> = Vec::new();
        for item_index in 0..self.items.len() {
            let item_value = self.items[item_index];
            copied_vector.push(item_value.to_vector());
        }

        copied_vector
    }
}

// impl Structifyable<Vec<Vec3<Vec3<f64>>>> for Mesh<Triangle> {
//     fn from_vector(vector: &Vec<Vec3<Vec3<f64>>>) -> Self {
//         let items = vector
//             .into_iter()
//             .map(|x| Triangle::from_vector(x))
//             .collect::<_>();
//         Mesh { items }
//     }
// }
impl Structifyable<Vec<Vec4<Vec4<f64>>>> for Mesh<Triangle> {
    fn from_vector(vector: &Vec<Vec4<Vec4<f64>>>) -> Self {
        let items = vector
            .into_iter()
            .map(|x| Triangle::from_vector(x))
            .collect::<_>();
        Mesh { items }
    }
}
