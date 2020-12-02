use ddd_rust_3d_graphics::{Point, Triangle};
use piston_window::types::Color;

pub fn get_cube_colors() -> [Color; 12] {
    [
        [1.0, 0.0, 0.0, 0.5],
        [0.0, 1.0, 0.0, 0.5],
        [0.0, 0.0, 1.0, 0.5],
        [1.0, 0.0, 0.0, 0.5],
        [0.0, 1.0, 0.0, 0.5],
        [0.0, 0.0, 1.0, 0.5],
        [1.0, 0.0, 0.0, 0.5],
        [0.0, 1.0, 0.0, 0.5],
        [0.0, 0.0, 1.0, 0.5],
        [1.0, 0.0, 0.0, 0.5],
        [0.0, 1.0, 0.0, 0.5],
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
