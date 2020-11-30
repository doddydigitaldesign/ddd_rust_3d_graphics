// #![feature(min_const_generics)]

extern crate ddd_rust_3d_graphics;

use ddd_rust_3d_graphics::{
    create_window, get_projection, into_polygons::IntoPolygons, traits::structifyable::*,
    traits::vectorizable::*, vector::Vec4, Mesh, Point, Triangle,
};

extern crate piston_window;
use piston_window::*;
extern crate vecmath;
use vecmath::col_mat4_transform;

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

    let projection = get_projection(F_ASPECT_RATIO, F_FIELD_OF_VIEW, F_FAR_PLANE, F_NEAR_PLANE);
    let translation = math::translate([100f64, 100f64]);

    let mut projected_triangles = [[[0f64; 4]; 4]; 12];
    for i in 0..cube_mesh.items.len() {
        let mesh_triangle = cube_mesh.items[i].to_vector();
        let mut projected_triangle: Vec4<Vec4<f64>> = [[0f64; 4]; 4];
        for j in 0..mesh_triangle.len() {
            let triangle_point = mesh_triangle[j];
            projected_triangle[j] = col_mat4_transform(
                projection,
                [
                    triangle_point[0],
                    triangle_point[1],
                    triangle_point[2],
                    1f64,
                ],
            );
        }
        projected_triangles[i] = projected_triangle;
    }

    // Open a window and draw stuff

    let mut window = create_window("DDD Rust 3D Graphics", (SCREEN_WIDTH, SCREEN_HEIGHT));
    let projected_mesh: Mesh<[Triangle; 12]> = Mesh::from_vector(&projected_triangles);
    let mut polygons1 = [[[0f64; 2]; 3]; 12];
    let mut polygons2 = [[[0f64; 2]; 3]; 12];

    // Add a projection of the cube to the window
    for i in 0..12 {
        let item = projected_mesh.items[i];
        polygons1[i] = item.into_polygon();
        for j in 0..3 {
            polygons1[i][j] = math::transform_pos(translation, polygons1[i][j]);
        }
    }

    // Add another, slightly bigger projection of the cube
    for i in 0..12 {
        let item = projected_mesh.items[i];
        polygons2[i] = item.into_polygon();

        for j in 0..3 {
            polygons2[i][j] = math::transform_vec(math::rotate_radians(180f64), polygons2[i][j]);
            polygons2[i][j] =
                math::transform_pos(math::translate([700f64, 700f64]), polygons2[i][j]);
        }
    }

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
