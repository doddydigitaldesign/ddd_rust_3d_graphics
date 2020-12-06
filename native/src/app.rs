use math::translate;
use obj::{Obj, ObjResult, Vertex};
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};

extern crate graphics;
use graphics::*;

use crate::{FPS_FACTOR, SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DrawBufferItem {
    pub(crate) vertex: Vertex,
    pub(crate) color: types::Color,
    pub(crate) transform: math::Matrix2d,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DrawBuffer {
    pub(crate) items: Vec<DrawBufferItem>,
    pub(crate) indices: Vec<u16>,
}

impl DrawBuffer {
    pub(crate) fn from_obj_file(obj: ObjResult<Obj<Vertex, u16>>) -> DrawBuffer {
        let (indices, vertices) = if let Ok(value) = obj {
            (value.indices, value.vertices)
        } else {
            (Vec::new(), Vec::new())
        };

        let mut items: Vec<DrawBufferItem> = Vec::new();

        vertices.iter().for_each(|vertex| {
            items.push(DrawBufferItem {
                vertex: *vertex,
                color: [1.0, 1.0, 1.0, 1.0],
                transform: translate([100.0, 100.0]),
            });
        });

        DrawBuffer { items, indices }
    }

    pub fn get_polygons<'a>(&mut self) -> Vec<[[f64; 2]; 1]> {
        let mut polygons: Vec<[[f64; 2]; 1]> = Vec::new();
        self.items.iter().for_each(|item| {
            let point3d = item.vertex.position;
            let projection = [[1f64, 0f64, 0f64], [0f64, 0f64, 1f64]];

            let mut projected: [f64; 2] = [0f64, 0f64];
            for (_i, coord) in point3d.iter().enumerate() {
                for (j, row) in projection.iter().enumerate() {
                    for (_k, elem) in row.iter().enumerate() {
                        projected[j] += (*coord as f64) * elem;
                    }
                }
            }

            let poly = [projected];

            polygons.push(poly);
        });

        polygons
    }
}

pub struct App {
    pub(crate) gl: GlGraphics, // OpenGL drawing backend.
    pub(crate) velocity: [f64; 2],
    pub(crate) position: [f64; 2],
    pub(crate) rotation: f64,
    pub(crate) angular_velocity: f64,
    pub(crate) radius: f64,
    pub(crate) color: [f32; 4],
}

impl App {
    pub(crate) fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let radius = self.radius;
        let color = self.color;
        let square = rectangle::square(0.0, 0.0, radius);
        let rotation = self.rotation;
        let (_x_screen_mid, _y_screen_mid) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let [x, y] = self.position;
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            println!("Middle of screen: {} x {}", _x_screen_mid, _y_screen_mid);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    pub(crate) fn update(&mut self, args: &UpdateArgs) {
        let [v_x, v_y] = self.velocity;
        let [x, y] = self.position;

        // Detect collision with walls
        let new_v_x = if x <= self.radius || x >= self.radius + SCREEN_WIDTH {
            -v_x
        } else {
            v_x
        };

        let new_v_y = if y <= self.radius || y >= self.radius + SCREEN_HEIGHT {
            -v_y
        } else {
            v_y
        };

        self.velocity = [new_v_x, new_v_y];

        // Update rotation
        self.rotation += self.angular_velocity * args.dt;

        // Update position
        self.position = [
            x + self.velocity[0] * args.dt * FPS_FACTOR,
            y + self.velocity[1] * args.dt * FPS_FACTOR,
        ];
    }
    // pub(crate) fn update(&mut self, _args: &UpdateArgs) {
    //     // Check collisions with screen edges
    //     let new_velocity_x =
    //         if self.velocity[0] > (SCREEN_WIDTH - self.radius) || self.velocity[0] <= self.radius {
    //             -self.velocity[0]
    //         } else {
    //             self.velocity[0]
    //         };

    //     let new_velocity_y = if self.velocity[1] > (SCREEN_HEIGHT - self.radius)
    //         || self.velocity[1] <= self.radius
    //     {
    //         -self.velocity[1]
    //     } else {
    //         self.velocity[1]
    //     };

    //     self.velocity = [new_velocity_x, new_velocity_y];

    //     // Update position
    //     self.position = [
    //         self.position[0] + self.velocity[0],
    //         self.position[1] + self.velocity[1],
    //     ];
    // }
}
