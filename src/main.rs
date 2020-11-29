#![feature(min_const_generics)]

extern crate ddd_rust_3d_graphics;
use ddd_rust_3d_graphics::{
    traits::structifyable::*, traits::vectorizable::*, vector::Vec4, Matrix, Mesh, Point, Triangle,
};

fn main() {
    // Create a cube mesh using triangles
    let triangles: Vec<Triangle> = vec![
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
    ];

    let mut cube_mesh: Mesh<Triangle> = Mesh::new(triangles);

    cube_mesh.items.push(Triangle {
        a: Point::new(3.0, 0.0, 0.0),
        b: Point::new(0.0, 3.0, 0.0),
        c: Point::new(0.0, 0.0, 3.0),
    });

    const SCREEN_HEIGHT: i64 = 1080;
    const SCREEN_WIDTH: i64 = 1920;
    const F_NEAR_PLANE: f64 = 0.1;
    const F_FAR_PLANE: f64 = 1000.0;
    const F_FIELD_OF_VIEW: f64 = 90.0;
    const F_ASPECT_RATIO: f64 = (SCREEN_WIDTH as f64) / (SCREEN_HEIGHT as f64);

    let f_field_of_view_radians: f64 =
        1.0 / (F_FIELD_OF_VIEW * 0.5 / 180.0 * std::f64::consts::PI).tan();

    let matrix: Matrix<4, 4> = Matrix::new();

    let projection: Matrix<4, 4> = matrix.get_projection(
        F_ASPECT_RATIO,
        f_field_of_view_radians,
        F_FAR_PLANE,
        F_NEAR_PLANE,
    );

    let mut projected_triangles = Vec::new();
    for mesh_triangle_index in 0..cube_mesh.items.len() {
        let mesh_triangle = cube_mesh.items[mesh_triangle_index].to_vector();
        let mut projected_triangle: Vec4<Vec4<f64>> = [[0f64; 4]; 4];
        for vertex_index in 0..mesh_triangle.len() {
            let triangle_point = &mesh_triangle[vertex_index];
            let vec4 = [triangle_point[0], triangle_point[1], triangle_point[2], 0.0];
            projected_triangle[vertex_index] = projection.multiply_vector(&vec4);
        }
        projected_triangles.push(projected_triangle);
    }

    let projected_mesh = Mesh::from_vector(&projected_triangles);
    println!("Projected Cube Mesh: {}", format!("{:?}", projected_mesh));
}
