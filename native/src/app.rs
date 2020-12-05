use math::translate;
use obj::{Obj, ObjResult, Vertex};
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};

extern crate graphics;
use graphics::*;

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
}

pub struct App {
    pub(crate) gl: GlGraphics, // OpenGL drawing backend.
}

impl App {
    pub(crate) fn render(&mut self, args: &RenderArgs, _draw_buffer: &DrawBuffer) {
        const BACKGROUND_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        self.gl.draw(args.viewport(), |_c, gl| {
            // Clear the screen.
            clear(BACKGROUND_COLOR, gl);
        });
    }

    pub(crate) fn update(&mut self, _args: &UpdateArgs) {
        // Check collisions
        // for item in self.draw_buffer.iter_mut() {
        //     todo!();
        // }
        // Check other interactions
    }
}
