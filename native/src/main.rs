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

use app::{App, DrawBuffer};
use shared::from_obj_file::get_obj_from_file;

pub(crate) const SCREEN_HEIGHT: f64 = 1080f64;
pub(crate) const SCREEN_WIDTH: f64 = 1920f64;
pub(crate) const WINDOW_TITLE: &str = "DDD Rust 3D Graphics";

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(WINDOW_TITLE, [SCREEN_WIDTH, SCREEN_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let obj = get_obj_from_file("shared/assets/cube3d.obj");

    let draw_buffer: DrawBuffer = DrawBuffer::from_obj_file(obj);

    let mut app = App {
        gl: GlGraphics::new(opengl),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &draw_buffer);
        } else if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
