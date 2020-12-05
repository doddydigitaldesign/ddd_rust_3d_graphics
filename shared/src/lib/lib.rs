#![feature(min_const_generics)]
#![feature(toowned_clone_into)]

pub mod matrix;
pub mod mesh;
pub mod plane;
pub mod point;
pub mod traits;
pub mod triangle;
pub mod types;
pub mod utils;
pub mod window;

pub use matrix::*;
pub use mesh::*;
pub use plane::*;
pub use point::*;
pub use traits::*;
pub use triangle::*;
pub use types::*;
pub use utils::*;
pub use window::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matrix_works() {
        let mat: Matrix<4, 4> = Matrix::new();
        let mat2: Matrix<4, 4> = Matrix::new();

        assert_eq!(mat, mat2);
    }

    #[test]
    fn mesh_works() {
        let mesh: Mesh<[Triangle; 12]> = Mesh::new(get_triangles());
        let mesh2: Mesh<[Triangle; 12]> = Mesh::new(get_triangles());

        assert_eq!(mesh, mesh2);
    }

    #[test]
    fn point_works() {
        let point = Point::new(0.0, 1.0, 2.0);
        let point2 = Point::new(0.0, 1.0, 2.0);

        assert_eq!(point, point2);
    }

    #[test]
    fn triangle_works() {
        let triangle = get_triangles()[0];
        let triangle2 = get_triangles()[0];

        assert_eq!(triangle, triangle2);
    }

    fn get_triangles() -> [Triangle; 12] {
        [
            // South
            Triangle::new(
                Point::new(0.0, 0.0, 0.0),
                Point::new(0.0, 1.0, 0.0),
                Point::new(1.0, 1.0, 0.0),
            ),
            Triangle::new(
                Point::new(0.0, 0.0, 0.0),
                Point::new(1.0, 1.0, 0.0),
                Point::new(1.0, 0.0, 0.0),
            ),
            // East
            Triangle::new(
                Point::new(1.0, 0.0, 0.0),
                Point::new(1.0, 1.0, 0.0),
                Point::new(1.0, 1.0, 1.0),
            ),
            Triangle::new(
                Point::new(1.0, 0.0, 0.0),
                Point::new(1.0, 1.0, 1.0),
                Point::new(1.0, 0.0, 1.0),
            ),
            // North
            Triangle::new(
                Point::new(1.0, 0.0, 1.0),
                Point::new(1.0, 1.0, 1.0),
                Point::new(0.0, 1.0, 1.0),
            ),
            Triangle::new(
                Point::new(1.0, 0.0, 1.0),
                Point::new(0.0, 1.0, 1.0),
                Point::new(0.0, 0.0, 1.0),
            ),
            // West
            Triangle::new(
                Point::new(0.0, 0.0, 1.0),
                Point::new(0.0, 1.0, 1.0),
                Point::new(0.0, 1.0, 0.0),
            ),
            Triangle::new(
                Point::new(0.0, 0.0, 1.0),
                Point::new(0.0, 1.0, 0.0),
                Point::new(0.0, 0.0, 0.0),
            ),
            // Top
            Triangle::new(
                Point::new(0.0, 1.0, 0.0),
                Point::new(0.0, 1.0, 1.0),
                Point::new(1.0, 1.0, 0.0),
            ),
            Triangle::new(
                Point::new(0.0, 1.0, 0.0),
                Point::new(1.0, 1.0, 1.0),
                Point::new(1.0, 1.0, 0.0),
            ),
            // Bottom
            Triangle::new(
                Point::new(1.0, 0.0, 1.0),
                Point::new(0.0, 0.0, 1.0),
                Point::new(0.0, 0.0, 0.0),
            ),
            Triangle::new(
                Point::new(1.0, 0.0, 1.0),
                Point::new(0.0, 0.0, 0.0),
                Point::new(1.0, 0.0, 0.0),
            ),
        ]
    }
}
