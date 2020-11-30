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
