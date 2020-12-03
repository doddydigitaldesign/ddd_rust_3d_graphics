#![feature(min_const_generics)]

extern crate ddd_rust_3d_graphics;
extern crate piston_window;
extern crate vecmath;

use ddd_rust_3d_graphics::{create_window, get_projection, Mesh, Triangle};

use piston_window::*;
use utils::{get_cube_colors, get_polygons_from_mesh, transform_cube_loop, transform_cube_scroll};

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
            Event::Input(_, _) => {
                e.mouse_scroll(|scroll_args| {
                    let transformed_cube = transform_cube_scroll(&mut cube_mesh, &scroll_args);
                    let polygons = get_polygons_from_mesh(&transformed_cube, &projection);
                    window.draw_2d(&e, |_c, g, _d| {
                        clear([1.0, 1.0, 1.0, 1.0], g);

                        for (i, polygon_triangle) in polygons.iter().enumerate() {
                            polygon(colors[i], polygon_triangle, _c.transform, g);
                        }
                    });
                });
            }
            Event::Loop(_) => {
                let transformed_cube = transform_cube_loop(&mut cube_mesh);
                let polygons = get_polygons_from_mesh(&transformed_cube, &projection);
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
