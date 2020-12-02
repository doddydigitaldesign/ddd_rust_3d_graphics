use piston_window::types::Polygons;

pub trait Drawable {
    fn into_drawable(&self) -> Polygons;
}
