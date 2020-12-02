// #![feature(min_const_generics)]
mod utils;
extern crate ddd_rust_3d_graphics;

use ddd_rust_3d_graphics::{
    create_window, get_projection, into_polygons::IntoPolygons, traits::structifyable::*,
    traits::vectorizable::*, vector::Vec4, Mesh, Triangle,
};

extern crate piston_window;
use piston_window::*;
extern crate vecmath;
// use types::Triangle;
use utils::get_cube_colors;
use vecmath::col_mat4_transform;

fn main() {
    // Create a cube mesh using triangles
    const NUMBER_OF_TRIANGLES: usize = 12;
    let triangles = utils::get_cube_triangles(0f64, 0f64, 0f64);
    let colors = get_cube_colors();
    let cube_mesh: Mesh<[Triangle; NUMBER_OF_TRIANGLES]> = Mesh::new(triangles);

    const SCREEN_HEIGHT: u64 = 1080u64;
    const SCREEN_WIDTH: u64 = 1920u64;
    const F_NEAR_PLANE: f64 = 1f64;
    const F_FAR_PLANE: f64 = 100f64;
    const F_FIELD_OF_VIEW: f64 = 70f64;
    const F_ASPECT_RATIO: f64 = (SCREEN_WIDTH as f64) / (SCREEN_HEIGHT as f64);

    let projection = get_projection(F_ASPECT_RATIO, F_FIELD_OF_VIEW, F_FAR_PLANE, F_NEAR_PLANE);

    let mut projected_triangles = [[[0f64; 4]; 4]; NUMBER_OF_TRIANGLES];
    for (i, mesh_triangle) in cube_mesh.items.iter().enumerate() {
        let mut projected_triangle: Vec4<Vec4<f64>> = [[0f64; 4]; 4];
        for (j, triangle_point) in mesh_triangle.to_vector().iter().enumerate() {
            let vec = [
                triangle_point[0],
                triangle_point[1],
                triangle_point[2],
                1f64,
            ];
            projected_triangle[j] = col_mat4_transform(projection, vec);
        }
        projected_triangles[i] = projected_triangle;
    }

    // Open a window and draw stuff

    let mut window = create_window("DDD Rust 3D Graphics", (SCREEN_WIDTH, SCREEN_HEIGHT));
    let projected_mesh: Mesh<[Triangle; NUMBER_OF_TRIANGLES]> =
        Mesh::from_vector(&projected_triangles);
    let mut polygons = [[[0f64; 2]; 3]; NUMBER_OF_TRIANGLES];

    for (i, elem) in projected_mesh.items.iter().enumerate() {
        let item = elem.into_polygon();

        for (j, point) in item.iter().enumerate() {
            polygons[i][j] =
                math::transform_pos(math::translate([1920f64 / 2f64, 1080f64 / 2f64]), *point);
        }
    }

    // Draw
    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g, _d| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            for (i, polygon_triangle) in polygons.iter().enumerate() {
                polygon(colors[i], polygon_triangle, _c.transform, g);
            }
        });
    }
}
