extern crate piston_window;
use piston_window::*;

pub fn create_window(title: &str, (width, height): (u64, u64)) -> PistonWindow {
    let settings = WindowSettings::new(title, (width as u32, height as u32)).exit_on_esc(true);
    let window: PistonWindow = settings
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    window
}
