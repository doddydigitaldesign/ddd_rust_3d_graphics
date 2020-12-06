#![feature(min_const_generics)]
#![feature(toowned_clone_into)]

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate shared;
extern crate vecmath;

use glutin_window::GlutinWindow as Window;

use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

mod app;
mod utils;

use app::App;

pub(crate) const SCREEN_HEIGHT: f64 = 1080f64;
pub(crate) const SCREEN_WIDTH: f64 = 1920f64;
pub(crate) const WINDOW_TITLE: &str = "DDD Rust 3D Graphics";
pub(crate) const FPS_FACTOR: f64 = 120.0;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(WINDOW_TITLE, [SCREEN_WIDTH, SCREEN_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        angular_velocity: 2.0,
        color: [1.0, 1.0, 1.0, 1.0],
        position: [960.0, 540.0],
        radius: 50.0,
        rotation: 0.0,
        velocity: [1.0, 1.0],
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

// fn main() {
//     // Change this to OpenGL::V2_1 if not working.
//     let opengl = OpenGL::V3_2;

//     // Create an Glutin window.
//     let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
//         .graphics_api(opengl)
//         .exit_on_esc(true)
//         .build()
//         .unwrap();

//     // Create a new game and run it.
//     let mut app = App {
//         gl: GlGraphics::new(opengl),
//         rotation: 0.0,
//     };

//     let mut events = Events::new(EventSettings::new());
//     while let Some(e) = events.next(&mut window) {
//         if let Some(args) = e.render_args() {
//             app.render(&args);
//         }

//         if let Some(args) = e.update_args() {
//             app.update(&args);
//         }
//     }
// }
