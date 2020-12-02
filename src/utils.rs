use ddd_rust_3d_graphics::{
    into_polygons::IntoPolygons, vector::Vec4, vectorizable::Vectorizable, Mesh, Point, Triangle,
};
use piston_window::{
    math::{self, rotate_radians, transform_pos},
    types::Color,
};
use vecmath::col_mat4_transform;

pub fn get_cube_colors() -> [Color; 12] {
    [
        [1.0, 0.0, 0.0, 0.5],
        [0.0, 1.0, 1.0, 0.5],
        [0.5, 0.2, 1.0, 0.5],
        [1.0, 0.0, 0.0, 0.5],
        [0.5, 1.0, 0.1, 0.5],
        [0.0, 0.5, 1.0, 0.5],
        [1.0, 0.0, 0.1, 0.5],
        [0.0, 1.0, 0.0, 0.5],
        [0.2, 0.0, 1.0, 0.5],
        [1.0, 0.0, 0.0, 0.5],
        [0.3, 1.0, 0.0, 0.5],
        [0.0, 0.0, 1.0, 0.5],
    ]
}

pub fn get_cube_triangles(origin_x: f64, origin_y: f64, origin_z: f64) -> [Triangle; 12] {
    [
        // South
        Triangle::new(
            Point::new(origin_x, origin_y, origin_z),
            Point::new(origin_x, origin_y + 1f64, origin_z),
            Point::new(origin_x + 1f64, origin_y + 1f64, origin_z),
        ),
        Triangle::new(
            Point::new(origin_x, origin_y, origin_z),
            Point::new(origin_x + 1f64, origin_y + 1f64, origin_z),
            Point::new(origin_x + 1f64, origin_y, origin_z),
        ),
        // East
        Triangle::new(
            Point::new(origin_x + 1f64, origin_y, origin_z),
            Point::new(origin_x + 1f64, origin_y + 1f64, origin_z),
            Point::new(origin_x + 1f64, origin_y + 1f64, origin_z + 1f64),
        ),
        Triangle::new(
            Point::new(origin_x + 1f64, origin_y, origin_z),
            Point::new(origin_x + 1f64, origin_y + 1f64, origin_z + 1f64),
            Point::new(origin_x + 1f64, origin_y, origin_z + 1f64),
        ),
        // North
        Triangle::new(
            Point::new(origin_x + 1f64, origin_y, origin_z + 1f64),
            Point::new(origin_x + 1f64, origin_y + 1f64, origin_z + 1f64),
            Point::new(origin_x, origin_y + 1f64, origin_z + 1f64),
        ),
        Triangle::new(
            Point::new(origin_x + 1f64, origin_y, origin_z + 1f64),
            Point::new(origin_x, origin_y + 1f64, origin_z + 1f64),
            Point::new(origin_x, origin_y, origin_z + 1f64),
        ),
        // West
        Triangle::new(
            Point::new(origin_x, origin_y, origin_z + 1f64),
            Point::new(origin_x, origin_y + 1f64, origin_z + 1f64),
            Point::new(origin_x, origin_y + 1f64, origin_z),
        ),
        Triangle::new(
            Point::new(origin_x, origin_y, origin_z + 1f64),
            Point::new(origin_x, origin_y + 1f64, origin_z),
            Point::new(origin_x, origin_y, origin_z),
        ),
        // Top
        Triangle::new(
            Point::new(origin_x, origin_y + 1f64, origin_z),
            Point::new(origin_x, origin_y + 1f64, origin_z + 1f64),
            Point::new(origin_x + 1f64, origin_y + 1f64, origin_z),
        ),
        Triangle::new(
            Point::new(origin_x, origin_y + 1f64, origin_z),
            Point::new(origin_x + 1f64, origin_y + 1f64, origin_z + 1f64),
            Point::new(origin_x + 1f64, origin_y + 1f64, origin_z),
        ),
        // Bottom
        Triangle::new(
            Point::new(origin_x + 1f64, origin_y, origin_z + 1f64),
            Point::new(origin_x, origin_y, origin_z + 1f64),
            Point::new(origin_x, origin_y, origin_z),
        ),
        Triangle::new(
            Point::new(origin_x + 1f64, origin_y, origin_z + 1f64),
            Point::new(origin_x, origin_y, origin_z),
            Point::new(origin_x + 1f64, origin_y, origin_z),
        ),
    ]
}

pub fn project_mesh<const N: usize>(
    mesh: &Mesh<[Triangle; N]>,
    projection: &[[f64; 4]; 4],
) -> [[[f64; 4]; 4]; N] {
    let mut projected_items = [[[0f64; 4]; 4]; N];
    for (i, mesh_triangle) in mesh.items.iter().enumerate() {
        let mut projected_item: Vec4<Vec4<f64>> = [[0f64; 4]; 4];
        for (j, triangle_point) in mesh_triangle.to_vector().iter().enumerate() {
            let vec = [
                triangle_point[0],
                triangle_point[1],
                triangle_point[2],
                1f64,
            ];
            projected_item[j] = col_mat4_transform(*projection, vec);
        }
        projected_items[i] = projected_item;
    }
    projected_items
}

pub fn get_transformed_mesh<const N: usize>(mesh: &Mesh<[Triangle; N]>) -> [[[f64; 2]; 3]; N] {
    let mut transformed_items = [[[0f64; 2]; 3]; N];
    for (i, elem) in mesh.items.iter().enumerate() {
        let item = elem.into_polygon();

        for (j, point) in item.iter().enumerate() {
            transformed_items[i][j] =
                math::transform_pos(math::translate([1920f64 / 2f64, 1080f64 / 2f64]), *point);
        }
    }
    transformed_items
}

pub fn transform_cube<const N: usize>(mesh: &mut Mesh<[Triangle; N]>) -> Mesh<[Triangle; N]> {
    let mut new_triangles: [Triangle; N] = [mesh.items[0]; N];
    for (i, elem) in mesh.items.iter_mut().enumerate() {
        let rotation_matrix_a = rotate_radians(0.01f64 / 180f64);
        let rotation_matrix_b = rotate_radians(0.01f64 / 180f64);
        let rotation_matrix_c = rotate_radians(0.01f64 / 180f64);

        let (a, b, c) = (elem.a, elem.b, elem.c);
        let a_vec = transform_pos(rotation_matrix_a, a.to_vector2());
        let b_vec = transform_pos(rotation_matrix_b, b.to_vector2());
        let c_vec = transform_pos(rotation_matrix_c, c.to_vector2());

        let triangle = Triangle::new(
            Point::new(a_vec[0], a_vec[1], a.z),
            Point::new(b_vec[0], b_vec[1], b.z),
            Point::new(c_vec[0], c_vec[1], c.z),
        );
        elem.a = triangle.a;
        elem.b = triangle.b;
        elem.c = triangle.c;

        new_triangles[i] = triangle;
    }
    Mesh::new(new_triangles)
}
