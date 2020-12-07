extern crate camera_controllers;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate vecmath;
#[macro_use]
extern crate gfx;
extern crate shader_version;

use camera_controllers::{
    model_view_projection, CameraPerspective, FirstPerson, FirstPersonSettings,
};
use gfx::traits::*;
use graphics::color::WHITE;
use obj::*;
use opengl_graphics::*;
use piston::input::*;
use piston_window::*;
use shader_version::glsl::GLSL;
use shader_version::Shaders;
use std::fs::File;
use std::io::BufReader;

pub(crate) const SCREEN_HEIGHT: f64 = 1080.0;
pub(crate) const SCREEN_WIDTH: f64 = 1920.0;
pub(crate) const WINDOW_TITLE: &str = "DDD Rust 3D Graphics";
pub(crate) const INITIAL_CAMERA_POSITION: [f32; 3] = [2.0, 2.0, 5.0];

gfx_vertex_struct!(Vertex {
    a_pos: [f32; 4] = "a_pos",
    a_tex_coord: [f32; 2] = "a_tex_coord",
});

impl Vertex {
    fn new(pos: [f32; 3], tc: [f32; 2]) -> Vertex {
        Vertex {
            a_pos: [pos[0], pos[1], pos[2], 1.0],
            a_tex_coord: tc,
        }
    }
}

gfx_pipeline!( pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    u_model_view_proj: gfx::Global<[[f32; 4]; 4]> = "u_model_view_proj",
    t_color: gfx::TextureSampler<[f32; 4]> = "t_color",
    out_color: gfx::RenderTarget<::gfx::format::Srgba8> = "o_Color",
    out_depth: gfx::DepthTarget<::gfx::format::DepthStencil> =
        gfx::preset::depth::LESS_EQUAL_WRITE,
});

fn get_glowing_cube_obj() -> Obj {
    let file = File::open("native/assets/glowing_cube.obj").unwrap();
    let input = BufReader::new(file);
    let model: Obj = load_obj(input).unwrap();

    model
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new(WINDOW_TITLE, [SCREEN_WIDTH, SCREEN_HEIGHT])
        .graphics_api(opengl)
        .samples(4)
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_capture_cursor(true);

    // Load model data from .obj file, exported from Blender
    let cube_obj = get_glowing_cube_obj();
    let vertex_data: Vec<Vertex> = cube_obj
        .vertices
        .into_iter()
        .map(|vert| Vertex::new(vert.position, [vert.normal[0], vert.normal[1]]))
        .collect();
    let index_data = cube_obj.indices.as_slice();

    let ref mut factory = window.factory.clone();

    let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, index_data);

    let texels = [
        [0xff, 0xff, 0xff, 0x00],
        [0xff, 0x00, 0xff, 0x00],
        [0x00, 0xff, 0x00, 0xff],
        [0xff, 0x00, 0xff, 0x00],
    ];
    let (_, texture_view) = factory
        .create_texture_immutable::<gfx::format::Rgba8>(
            gfx::texture::Kind::D2(2, 2, gfx::texture::AaMode::Single),
            gfx::texture::Mipmap::Provided,
            &[&texels],
        )
        .unwrap();

    let sinfo = gfx::texture::SamplerInfo::new(
        gfx::texture::FilterMethod::Trilinear,
        gfx::texture::WrapMode::Clamp,
    );

    let glsl = opengl.to_glsl();
    let pipeline_state = factory
        .create_pipeline_simple(
            Shaders::new()
                .set(GLSL::V1_20, include_str!("../assets/cube_120.glslv"))
                .set(GLSL::V1_50, include_str!("../assets/cube_150.glslv"))
                .get(glsl)
                .unwrap()
                .as_bytes(),
            Shaders::new()
                .set(GLSL::V1_20, include_str!("../assets/cube_120.glslf"))
                .set(GLSL::V1_50, include_str!("../assets/cube_150.glslf"))
                .get(glsl)
                .unwrap()
                .as_bytes(),
            pipe::new(),
        )
        .unwrap();

    let get_projection = |w: &PistonWindow| {
        let draw_size = w.window.draw_size();
        CameraPerspective {
            fov: 70.0,
            near_clip: 0.1,
            far_clip: 1000.0,
            aspect_ratio: (draw_size.width as f32) / (draw_size.height as f32),
        }
        .projection()
    };

    let model = vecmath::mat4_id();
    let mut projection = get_projection(&window);
    let mut first_person_camera = FirstPerson::new(
        INITIAL_CAMERA_POSITION,
        FirstPersonSettings::keyboard_wasd(),
    );

    let mut data = pipe::Data {
        vbuf: vbuf.clone(),
        u_model_view_proj: [[0.0; 4]; 4],
        t_color: (texture_view, factory.create_sampler(sinfo)),
        out_color: window.output_color.clone(),
        out_depth: window.output_stencil.clone(),
    };

    // let texture_settings = TextureSettings::new().filter(Filter::Nearest);

    // let ref mut glyphs =
    //     GlyphCache::new("native/assets/FiraSans-Regular.ttf", (), texture_settings)
    //         .expect("Could not load font");

    while let Some(e) = window.next() {
        first_person_camera.event(&e);

        window.draw_3d(&e, |window| {
            let args = e.render_args().unwrap();

            window
                .encoder
                .clear(&window.output_color, [0.3, 0.3, 0.3, 1.0]);
            window.encoder.clear_depth(&window.output_stencil, 1.0);

            data.u_model_view_proj = model_view_projection(
                model,
                first_person_camera.camera(args.ext_dt).orthogonal(),
                projection,
            );
            window.encoder.draw(&slice, &pipeline_state, &data);
        });

        // window.draw_2d(&e, |c, gl_graphics: &mut Graphics<Texture = <C as CharacterCache>::Texture>, device| {
        //     let text_title_transform = c.transform.trans(100.0, 100.0);
        //     let text_keys_transform = c.transform.trans(100.0, 150.0);
        //     let text_title = "Demo";
        //     let text_keys = "Keybindings: Acceleration - WASD, Break - Spacebar, ESC - Quit";

        //     graphics::text::Text::new_color(WHITE, 32)
        //         .draw(
        //             text_title,
        //             glyphs,
        //             &c.draw_state,
        //             text_title_transform,
        //             gl_graphics,
        //         )
        //         .ok();
        //     graphics::text::Text::new_color(WHITE, 24)
        //         .draw(
        //             text_keys,
        //             glyphs,
        //             &c.draw_state,
        //             text_keys_transform,
        //             gl_graphics,
        //         )
        //         .ok();
        // });

        if let Some(_) = e.resize_args() {
            projection = get_projection(&window);
            data.out_color = window.output_color.clone();
            data.out_depth = window.output_stencil.clone();
        }
    }
}
