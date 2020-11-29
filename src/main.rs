// #![feature(min_const_generics)]

extern crate ddd_rust_3d_graphics;
use ddd_rust_3d_graphics::{
    create_window, into_polygons::IntoPolygons, scalable::Scalable, traits::structifyable::*,
    traits::vectorizable::*, vector::Vec4, Matrix, Mesh, Point, Triangle,
};

extern crate piston_window;
use piston_window::*;
fn main() {
    // Create a cube mesh using triangles
    let triangles: [Triangle; 12] = [
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

    let cube_mesh: Mesh<[Triangle; 12]> = Mesh::new(triangles);

    const SCREEN_HEIGHT: u64 = 1080;
    const SCREEN_WIDTH: u64 = 1920;
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

    let mut projected_triangles = [[[0f64; 4]; 4]; 12];
    for mesh_triangle_index in 0..cube_mesh.items.len() {
        let mesh_triangle = cube_mesh.items[mesh_triangle_index].to_vector();
        let mut projected_triangle: Vec4<Vec4<f64>> = [[0f64; 4]; 4];
        for vertex_index in 0..mesh_triangle.len() {
            let triangle_point = &mesh_triangle[vertex_index];
            let vec4 = [triangle_point[0], triangle_point[1], triangle_point[2], 0.0];
            projected_triangle[vertex_index] = projection.multiply_vector(&vec4);
        }
        projected_triangles[mesh_triangle_index] = projected_triangle;
    }

    // Open a window and draw stuff

    let mut window = create_window("DDD Rust 3D Graphics", (SCREEN_WIDTH, SCREEN_HEIGHT));

    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g, _d| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            let projected_mesh: Mesh<[Triangle; 12]> = Mesh::from_vector(&projected_triangles);
            let mut polygons = [[[0f64; 2]; 3]; 12];
            for i in 0..12 {
                let mut item = projected_mesh.items[i];
                polygons[i] = item.scale(100f64).into_polygon();
                polygon([0.4, 0.0, 0.5, 1.0], &polygons[i], _c.transform, g);
            }
        });
    }
}
