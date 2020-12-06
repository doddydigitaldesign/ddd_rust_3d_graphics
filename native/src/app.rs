use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use graphics::{CharacterCache, Graphics};
use opengl_graphics::GlGraphics;
use piston::{Key, RenderArgs, UpdateArgs};

use crate::{ACCELERATION_FACTOR, BALL_RADIUS, FPS_FACTOR, RADIUS, SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct App {
    pub(crate) gl: GlGraphics, // OpenGL drawing backend.
    pub(crate) velocity: [f64; 2],
    pub(crate) position: [f64; 2],
    pub(crate) rotation: f64,
    pub(crate) angular_velocity: f64,
    pub(crate) radius: f64,
    pub(crate) color: [f32; 4],
    pub(crate) fps: usize,
    pub(crate) last_second_frames: VecDeque<Instant>,
}

impl App {
    pub(crate) fn render<G: Graphics, C>(&mut self, args: &RenderArgs, glyphs: &mut C)
    where
        C: CharacterCache<Texture = opengl_graphics::Texture>,
    {
        use graphics::*;

        self.fps = self.tick();

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BALL_COLOR: [f32; 4] = [0.0, 0.0, 0.55, 0.3];

        let radius = self.radius;
        let color = self.color;
        let square = graphics::rectangle::square(0.0, 0.0, radius * 2.0);
        let ball = graphics::ellipse::circle(0.0, 0.0, BALL_RADIUS);
        let rotation = self.rotation;
        let (_x_screen_mid, _y_screen_mid) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let [x, y] = self.position;
        let speed = (self.velocity[0].powi(2) + self.velocity[1].powi(2)).sqrt();
        let fps = self.fps;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            // println!("Square position: {} x {}", x, y);

            let square_transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-radius, -radius);
            let ball_transform = c.transform.trans(_x_screen_mid, _y_screen_mid);
            let text_title_transform = c.transform.trans(100.0, 100.0);
            let text_ball_transform = c.transform.trans(
                args.window_size[0] / 2.0 - BALL_RADIUS / 2.0,
                args.window_size[1] / 2.0,
            );
            let text_keys_transform = c.transform.trans(100.0, 150.0);
            let text_speed_transform = c.transform.trans(100.0, 200.0);
            let text_position_transform = c.transform.trans(100.0, 250.0);
            let text_fps_transform = c.transform.trans(100.0, 300.0);

            let text_title = "Demo";
            let text_ball = "Drag applied here!";
            let text_keys = "Keybindings: Acceleration - WASD, Break - Spacebar, ESC - Quit";
            let text_position = format!("Position x: {} , y: {}", x.round(), y.round());
            let text_speed = format!("Speed: {}", speed.round());
            let text_fps = format!("FPS: {}", fps);

            rectangle(color, square, square_transform, gl);
            ellipse(BALL_COLOR, ball, ball_transform, gl);

            graphics::text::Text::new_color(WHITE, 32)
                .draw(text_title, glyphs, &c.draw_state, text_title_transform, gl)
                .ok();
            graphics::text::Text::new_color(WHITE, 24)
                .draw(text_ball, glyphs, &c.draw_state, text_ball_transform, gl)
                .ok();
            graphics::text::Text::new_color(WHITE, 24)
                .draw(text_keys, glyphs, &c.draw_state, text_keys_transform, gl)
                .ok();
            graphics::text::Text::new_color(WHITE, 24)
                .draw(
                    text_position.as_str(),
                    glyphs,
                    &c.draw_state,
                    text_position_transform,
                    gl,
                )
                .ok();
            graphics::text::Text::new_color(WHITE, 24)
                .draw(
                    text_speed.as_str(),
                    glyphs,
                    &c.draw_state,
                    text_speed_transform,
                    gl,
                )
                .ok();
            graphics::text::Text::new_color(WHITE, 24)
                .draw(
                    text_fps.as_str(),
                    glyphs,
                    &c.draw_state,
                    text_fps_transform,
                    gl,
                )
                .ok();
        });
    }

    pub(crate) fn update(&mut self, args: &UpdateArgs) {
        let [x, y] = self.position;
        let inside_ball = is_inside_ball([x, y]);

        let [v_x, v_y] = self.velocity;
        let speed = (v_x.powi(2) + v_y.powi(2)).sqrt();

        // Update scale
        self.radius = RADIUS + (if speed >= 50.0 { 50.0 } else { speed });
        let area = 2.0 * self.radius;

        // Detect collision with walls
        let border_left = self.radius;
        let border_right = SCREEN_WIDTH - self.radius;
        let border_bottom = SCREEN_HEIGHT - self.radius;
        let border_top = self.radius;
        fn is_inside_ball(pos: [f64; 2]) -> bool {
            let vector_to_center = [980.0 - pos[0], 540.0 - pos[1]];
            let distance_from_center =
                ((vector_to_center[0]).powi(2) + (vector_to_center[1]).powi(2)).sqrt();

            distance_from_center <= BALL_RADIUS
        }
        let drag_x = 0.5 * (1.0 / v_x.abs()) * v_x.powi(2) * area * args.dt;
        let drag_y = 0.5 * (1.0 / v_y.abs()) * v_y.powi(2) * area * args.dt;

        let new_v_x = if x <= border_left || x >= border_right {
            -v_x * args.dt * FPS_FACTOR
        } else {
            let mut value = v_x;
            if inside_ball {
                if value > 1.0 {
                    value = value - drag_x;
                } else if value < -1.0 {
                    value = value + drag_x;
                }
            }

            value * args.dt * FPS_FACTOR
        };

        let new_v_y = if y <= border_top || y >= border_bottom {
            -v_y * args.dt * FPS_FACTOR
        } else {
            let mut value = v_y;
            if inside_ball {
                if value > 1.0 {
                    value = value - drag_y
                } else if value < -1.0 {
                    value = value + drag_y
                }
            }

            value * args.dt * FPS_FACTOR
        };

        // Update velocity
        self.velocity = [new_v_x, new_v_y];

        // Update color
        let red_factor = ((new_v_x.abs()) / (1.0 + new_v_x.abs() + new_v_y.abs())) as f32
            + ((new_v_y.abs()) / (1.0 + new_v_x.abs() + new_v_y.abs())) as f32;
        let blue_factor = (1.0f32 - red_factor).abs();
        let green_factor = red_factor / 4.0;
        self.color = [red_factor, green_factor, blue_factor, 1.0];

        // Update rotation
        self.rotation += self.angular_velocity * args.dt;

        // Update position
        self.position = [x + self.velocity[0], y + self.velocity[1]];
    }

    pub(crate) fn update_on_input(&mut self, key: Key) {
        let [v_x, v_y] = self.velocity;

        if key == Key::A {
            self.velocity = [v_x - 1.0 * ACCELERATION_FACTOR, v_y];
        }
        if key == Key::D {
            self.velocity = [v_x + 1.0 * ACCELERATION_FACTOR, v_y];
        }
        if key == Key::W {
            self.velocity = [v_x, v_y - 1.0 * ACCELERATION_FACTOR];
        }
        if key == Key::S {
            self.velocity = [v_x, v_y + 1.0 * ACCELERATION_FACTOR];
        }
        if key == Key::Space {
            self.velocity = [v_x / ACCELERATION_FACTOR, v_y / ACCELERATION_FACTOR];
        }
    }
    pub fn tick(&mut self) -> usize {
        let now = Instant::now();
        let a_second_ago = now - Duration::from_secs(1);

        while self
            .last_second_frames
            .front()
            .map_or(false, |t| *t < a_second_ago)
        {
            self.last_second_frames.pop_front();
        }

        self.last_second_frames.push_back(now);
        self.last_second_frames.len()
    }
}
