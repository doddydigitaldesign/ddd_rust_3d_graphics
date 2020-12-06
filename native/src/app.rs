use opengl_graphics::GlGraphics;
use piston::{Key, RenderArgs, UpdateArgs};

use crate::{ACCELERATION_FACTOR, FPS_FACTOR, SCREEN_HEIGHT, SCREEN_WIDTH};

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

        let radius = self.radius;
        let color = self.color;
        let square = rectangle::square(0.0, 0.0, radius * 2.0);
        let rotation = self.rotation;
        let (_x_screen_mid, _y_screen_mid) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let [x, y] = self.position;
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            println!("Square position: {} x {}", x, y);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-radius, -radius);

            // Draw a box rotating around the middle of the screen.
            rectangle(color, square, transform, gl);
        });
    }

    pub(crate) fn update(&mut self, args: &UpdateArgs) {
        let [v_x, v_y] = self.velocity;
        let [x, y] = self.position;

        // Detect collision with walls
        let border_left = self.radius;
        let border_right = SCREEN_WIDTH - self.radius;
        let border_bottom = SCREEN_HEIGHT - self.radius;
        let border_top = self.radius;

        let new_v_x = if x <= border_left || x >= border_right {
            -v_x * args.dt * FPS_FACTOR
        } else {
            v_x * args.dt * FPS_FACTOR
        };

        let new_v_y = if y <= border_top || y >= border_bottom {
            -v_y * args.dt * FPS_FACTOR
        } else {
            v_y * args.dt * FPS_FACTOR
        };

        self.velocity = [new_v_x, new_v_y];

        // Update rotation
        self.rotation += self.angular_velocity * args.dt;

        // Update position
        self.position = [x + self.velocity[0], y + self.velocity[1]];
    }

    pub(crate) fn update_on_input(&mut self, key: Key) {
        let [v_x, v_y] = self.velocity;

        if key == Key::A {
            self.velocity = [v_x - ACCELERATION_FACTOR, v_y];
        }
        if key == Key::D {
            self.velocity = [v_x + ACCELERATION_FACTOR, v_y];
        }
        if key == Key::W {
            self.velocity = [v_x, v_y - ACCELERATION_FACTOR];
        }
        if key == Key::S {
            self.velocity = [v_x, v_y + ACCELERATION_FACTOR];
        }
    }
}
