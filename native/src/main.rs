#![feature(min_const_generics)]
#![feature(toowned_clone_into)]

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate vecmath;

use glutin_window::GlutinWindow as AppWindow;

use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::{AdvancedWindow, Window, WindowSettings};

mod app;
mod utils;

use app::App;

pub(crate) const SCREEN_HEIGHT: f64 = 1080f64;
pub(crate) const SCREEN_WIDTH: f64 = 1920f64;
pub(crate) const SCREEN_MID_POINT: [f64; 2] = [SCREEN_HEIGHT / 2.0, SCREEN_WIDTH / 2.0];
pub(crate) const WINDOW_TITLE: &str = "DDD Rust 3D Graphics";
pub(crate) const FPS_FACTOR: f64 = 120.0;
pub(crate) const MAX_FPS: u64 = 120;
pub(crate) const ACCELERATION_FACTOR: f64 = 1.0;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: AppWindow = WindowSettings::new(WINDOW_TITLE, [SCREEN_WIDTH, SCREEN_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let gl = GlGraphics::new(opengl);
    let mut app = App {
        gl,
        angular_velocity: 2.0,
        color: [1.0, 1.0, 1.0, 1.0],
        position: SCREEN_MID_POINT,
        radius: 50.0,
        rotation: 0.0,
        velocity: [1.0, 1.0],
    };

    let mut events = Events::new(EventSettings {
        bench_mode: false,  // Default false
        lazy: false,        // Default false
        max_fps: MAX_FPS,   // Default 60
        ups: 120,           // Default 120
        ups_reset: 2,       // Default 2
        swap_buffers: true, // Default true
    });
    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.update_on_input(key);
        }

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
