use camera_controllers::CameraPerspective;

use crate::{coordinate_system::Coordinate, matrix_sizes::Mat4By4};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Matrix<const R: usize, const C: usize> {
    pub data: [[f64; R]; C],
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn new() -> Matrix<R, C> {
        Matrix {
            data: [[0f64; R]; C],
        }
    }

    pub fn get_rotation(&self, axis: Coordinate, angle: f64) -> Mat4By4 {
        match axis {
            Coordinate::X => {
                return Matrix {
                    data: [
                        [1f64, 0f64, 0f64, 0f64],
                        [0f64, angle.cos(), -angle.sin(), 0f64],
                        [0f64, angle.sin(), angle.cos(), 0f64],
                        [0f64, 0f64, 0f64, 0f64],
                    ],
                }
            }
            Coordinate::Y => {
                return Matrix {
                    data: [
                        [angle.cos(), 0f64, angle.sin(), 0f64],
                        [0f64, 1f64, 0f64, 0f64],
                        [-angle.sin(), 0f64, angle.cos(), 0f64],
                        [0f64, 0f64, 0f64, 0f64],
                    ],
                }
            }
            Coordinate::Z => {
                return Matrix {
                    data: [
                        [angle.cos(), -angle.sin(), 0f64, 0f64],
                        [angle.sin(), angle.cos(), 0f64, 0f64],
                        [0f64, 0f64, 1f64, 0f64],
                        [0f64, 0f64, 0f64, 0f64],
                    ],
                }
            }
        }
    }

    pub fn multiply_vector(&self, vector: &[f64; R]) -> [f64; R] {
        let mut output_vec = [0.0; R];
        for vec_index in 0..R {
            let vec_entry = vector[vec_index];
            let matrix_row = &self.data[vec_index];
            let mut sum_of_cols: f64 = 0.0;
            for row_index in 0..matrix_row.len() {
                sum_of_cols += matrix_row[row_index] * vec_entry;
            }
            output_vec[vec_index] = sum_of_cols;
        }

        return output_vec;
    }
}

pub fn get_projection(
    f_aspect_ratio: f64,
    f_field_of_view_radians: f64,
    f_far_plane: f64,
    f_near_plane: f64,
) -> [[f64; 4]; 4] {
    let projection = CameraPerspective {
        aspect_ratio: f_aspect_ratio,
        far_clip: f_far_plane,
        fov: f_field_of_view_radians * 180f64,
        near_clip: f_near_plane,
    }
    .projection();
    projection
}

pub fn get_transposed<const M: usize, const N: usize>(matrix: &[[f64; M]; N]) -> [[f64; N]; M] {
    let mut transposed: [[f64; N]; M] = [[0f64; N]; M];
    // i < N
    for (i_rows, row_i) in matrix.iter().enumerate() {
        // j < M
        for (i_cols, col_i) in row_i.iter().enumerate() {
            transposed[i_cols][i_rows] = 0f64 + col_i;
        }
    }

    transposed
}

pub fn matrix_mul_self<const M: usize, const N: usize>(matrix: &[[f64; M]; N]) -> [[f64; M]; M] {
    let mut out: [[f64; M]; M] = [[0f64; M]; M];

    let transposed = get_transposed::<M, N>(matrix);

    for i in 0..transposed.len() {
        for j in 0..transposed.len() {
            out[i][j] = dot_product(&transposed[j], &transposed[i]);
        }
    }

    out
}

pub fn dot_product<const N: usize>(vector1: &[f64; N], vector2: &[f64; N]) -> f64 {
    let mut out: f64 = 0f64;

    for (i, elem) in vector1.iter().enumerate() {
        out += elem * vector2[i];
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matrix_transpose_works() {
        let matrix: [[f64; 3]; 2] = [[1f64, 2f64, 3f64], [4f64, 5f64, 6f64]];

        let result = get_transposed::<3, 2>(&matrix);

        let correct = [[1f64, 4f64], [2f64, 5f64], [3f64, 6f64]];

        assert_eq!(result, correct);
    }
    #[test]
    fn matrix_mul_self_works() {
        let matrix: [[f64; 3]; 2] = [[1f64, 2f64, 3f64], [4f64, 5f64, 6f64]];

        let result = matrix_mul_self(&matrix);

        let correct = [
            [17f64, 22f64, 27f64],
            [22f64, 29f64, 36f64],
            [27f64, 36f64, 45f64],
        ];

        assert_eq!(result, correct);
    }
}
