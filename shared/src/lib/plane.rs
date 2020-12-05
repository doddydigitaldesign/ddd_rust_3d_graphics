use crate::vector::Vec3;
use vecmath::vec3_cross;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Plane {
    pub width: f64,
    pub height: f64,
    /// Perpendicular vector to the plane
    pub normal: Vec3<f64>,
    /// Point on the plane
    pub origin: Vec3<f64>,
}

impl Plane {
    pub fn new(width: f64, height: f64, normal: Vec3<f64>, origin: Vec3<f64>) -> Self {
        Plane {
            width,
            height,
            normal,
            origin,
        }
    }
    /// Get a vector basis pair for the plane
    pub fn get_basis(&self) -> [Vec3<f64>; 2] {
        let vec_normal_to_origin = [
            self.origin[0] - self.normal[0],
            self.origin[1] - self.normal[1],
            self.origin[2] - self.normal[2],
        ];
        let vec_normal_to_origin_orthogonal: Vec3<f64> =
            vec3_cross(self.normal, vec_normal_to_origin);

        [vec_normal_to_origin, vec_normal_to_origin_orthogonal]
    }
    /// Orthogonal projection of a vector onto the Plane
    // pub fn project(&self, vector: Vec3<f64>) -> Vec2<f64> {
    //     // Given the matrix A, [basis_vec1, basis_vec2]
    //     // projection matrix is A * (A.transpose() * A).inverse() * A.transpose()
    //     let matrix_a = self.get_basis();
    //     let matrix_a_inverse_d = mat2x3_transposed(matrix_a);
    //     let matrix_a_transposed_times_a = col_mat3x2_mul(matrix_a_transposed, matrix_a);
    // }
    pub fn has_point(&self, point: Vec3<f64>) -> bool {
        let (a, b, c) = (self.normal[0], self.normal[1], self.normal[2]);
        let (x0, y0, z0) = (self.origin[0], self.origin[1], self.origin[2]);
        let (x, y, z) = (point[0], point[1], point[2]);
        a * (x - x0) + b * (y - y0) + c * (z - z0) == 0f64
    }
}
