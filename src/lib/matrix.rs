use crate::matrix_sizes::Mat4By4;

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

    pub fn get_projection(
        &self,
        f_aspect_ratio: f64,
        f_field_of_view_radians: f64,
        f_far_plane: f64,
        f_near_plane: f64,
    ) -> Mat4By4 {
        let mut data = [[0f64; 4]; 4];

        data[0][0] = f_aspect_ratio * f_field_of_view_radians;
        data[1][1] = f_field_of_view_radians;
        data[2][2] = f_far_plane / (f_far_plane - f_near_plane);
        data[3][2] = (-f_far_plane * f_near_plane) / (f_far_plane - f_near_plane);
        data[2][3] = 1.0f64;
        data[3][3] = 0.0f64;

        return Matrix { data };
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
