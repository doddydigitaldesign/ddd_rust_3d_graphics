// #![feature(min_const_generics)]

extern crate ddd_rust_3d_graphics;
use ddd_rust_3d_graphics::{
    coordinate_system::Coordinate, create_window, into_polygons::IntoPolygons, scalable::Scalable,
    traits::structifyable::*, traits::vectorizable::*, vector::Vec4, Matrix, Mesh, Point, Triangle,
};

extern crate piston_window;
use piston_window::*;

pub fn get_cube_triangles() -> [Triangle; 12] {
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
    triangles
}

fn main() {
    // Create a cube mesh using triangles
    let triangles = get_cube_triangles();
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

    let rotation_x: Matrix<4, 4> = matrix.get_rotation(Coordinate::X, 45f64);
    let rotation_y: Matrix<4, 4> = matrix.get_rotation(Coordinate::Y, 45f64);

    // let translation = Point::new(100f64, 100f64, 0f64);
    let translation = math::translate([100f64, 100f64]);

    let mut projected_triangles = [[[0f64; 4]; 4]; 12];
    for mesh_triangle_index in 0..cube_mesh.items.len() {
        let mesh_triangle = cube_mesh.items[mesh_triangle_index].to_vector();
        let mut projected_triangle: Vec4<Vec4<f64>> = [[0f64; 4]; 4];
        for vertex_index in 0..mesh_triangle.len() {
            let triangle_point = &mesh_triangle[vertex_index];
            let vec4 = [triangle_point[0], triangle_point[1], triangle_point[2], 0.0];
            let vec4_rotated = rotation_y.multiply_vector(&vec4);
            let vec4_projected = projection.multiply_vector(&vec4_rotated);
            println!(
                "original vector:{}\n rotated vector: {}\n projected vector: {}",
                format!("{:?}", vec4),
                format!("{:?}", vec4_rotated),
                format!("{:?}", vec4_projected)
            );
            projected_triangle[vertex_index] = vec4_projected;
        }
        projected_triangles[mesh_triangle_index] = projected_triangle;
    }

    // Open a window and draw stuff

    let mut window = create_window("DDD Rust 3D Graphics", (SCREEN_WIDTH, SCREEN_HEIGHT));
    let projected_mesh: Mesh<[Triangle; 12]> = Mesh::from_vector(&projected_triangles);
    let mut polygons1 = [[[0f64; 2]; 3]; 12];
    let mut polygons2 = [[[0f64; 2]; 3]; 12];

    // Add a projection of the cube to the window
    for i in 0..12 {
        let mut item = projected_mesh.items[i];
        polygons1[i] = item.scale(100f64).into_polygon();
        polygons1[i][0] = math::transform_pos(translation, polygons1[i][0]);
        polygons1[i][1] = math::transform_pos(translation, polygons1[i][1]);
        polygons1[i][2] = math::transform_pos(translation, polygons1[i][2]);
    }

    // Add another, slightly bigger projection of the cube
    for i in 0..12 {
        let mut item = projected_mesh.items[i];
        polygons2[i] = item.scale(200f64).into_polygon();

        polygons2[i][0] = math::transform_vec(math::rotate_radians(45f64), polygons2[i][0]);
        polygons2[i][1] = math::transform_vec(math::rotate_radians(45f64), polygons2[i][1]);
        polygons2[i][2] = math::transform_vec(math::rotate_radians(45f64), polygons2[i][2]);

        polygons2[i][0] = math::transform_pos(math::translate([600f64, 100f64]), polygons2[i][0]);
        polygons2[i][1] = math::transform_pos(math::translate([600f64, 100f64]), polygons2[i][1]);
        polygons2[i][2] = math::transform_pos(math::translate([600f64, 100f64]), polygons2[i][2]);
    }
    // println!("polygons: {}", format!("{:#?}", polygons));

    // Draw
    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g, _d| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            for i in 0..12 {
                polygon([0.4, 0.0, 0.5, 1.0], &polygons1[i], _c.transform, g);
            }
            for i in 0..12 {
                polygon([0.3, 0.0, 0.5, 1.0], &polygons2[i], _c.transform, g);
            }
        });
    }
}
