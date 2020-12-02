#![feature(min_const_generics)]

extern crate ddd_rust_3d_graphics;
extern crate piston_window;
extern crate vecmath;

use ddd_rust_3d_graphics::{
    create_window, get_projection, traits::structifyable::*, Mesh, Triangle,
};

use piston_window::*;
use utils::{get_cube_colors, get_transformed_mesh, project_mesh, transform_cube};

mod utils;

fn main() {
    const SCREEN_HEIGHT: u64 = 1080u64;
    const SCREEN_WIDTH: u64 = 1920u64;
    const F_NEAR_PLANE: f64 = 0.01f64;
    const F_FAR_PLANE: f64 = 1000f64;
    const F_FIELD_OF_VIEW: f64 = 120f64;
    const F_ASPECT_RATIO: f64 = (SCREEN_WIDTH as f64) / (SCREEN_HEIGHT as f64);
    let mut window = create_window("DDD Rust 3D Graphics", (SCREEN_WIDTH, SCREEN_HEIGHT));

    let projection = get_projection(F_ASPECT_RATIO, F_FIELD_OF_VIEW, F_FAR_PLANE, F_NEAR_PLANE);
    // Create a cube mesh using triangles
    const NUMBER_OF_TRIANGLES: usize = 12;
    let triangles = utils::get_cube_triangles(0f64, 0f64, 1000f64);
    let colors = get_cube_colors();
    let mut cube_mesh: Mesh<[Triangle; NUMBER_OF_TRIANGLES]> = Mesh::new(triangles);

    // Draw
    while let Some(e) = window.next() {
        match e {
            Event::Input(_, _) => {}
            Event::Loop(_) => {
                let transformed_cube = transform_cube(&mut cube_mesh);
                let projected_triangles = project_mesh(&transformed_cube, &projection);
                let projected_mesh: Mesh<[Triangle; NUMBER_OF_TRIANGLES]> =
                    Mesh::from_vector(&projected_triangles);
                let polygons = get_transformed_mesh(&projected_mesh);
                window.draw_2d(&e, |_c, g, _d| {
                    clear([1.0, 1.0, 1.0, 1.0], g);

                    for (i, polygon_triangle) in polygons.iter().enumerate() {
                        polygon(colors[i], polygon_triangle, _c.transform, g);
                    }
                });
            }
            Event::Custom(_, _, _) => {}
        }
    }
}
