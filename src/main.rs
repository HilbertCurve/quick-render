extern crate gl;
extern crate glfw;

use std::error::Error;

use glam::{Vec3, Vec4, vec4, vec3};
use glfw::{Action, Context, Key};

use crate::{
    buffer::{VertexBuffer, VProp, VType},
    renderer::{DEFAULT_VB, Renderable, RenderError},
};

pub mod buffer;
pub mod camera;
pub mod mouse;
pub mod primitive;
pub mod renderer;
pub mod shader;
pub mod utils;
pub mod window;

// TODO: move inside main()
static mut T1: f64 = 0.0;
static mut T2: f64 = 0.0;
static mut DT: f64 = 0.0;

struct Rect {
    pub pos: Vec3,
    pub whd: Vec3,
    pub color: Vec4,
}

impl Renderable for Rect {
    fn to_buffer(&self, buf: &mut VertexBuffer) -> Result<(), renderer::RenderError> {
        let mut offset = buf.vb.len();

        // get attrib info and do checks
        let (pos_pos, pos_size, pos_type_enum) = buf.attrib_metadata(VProp::Position)?;
        let (col_pos, col_size, col_type_enum) = buf.attrib_metadata(VProp::Color)?;
        let (tuv_pos, tuv_size, tuv_type_enum) = buf.attrib_metadata(VProp::TexUV)?;
        let (tid_pos, _, tid_type_enum) = buf.attrib_metadata(VProp::TexID)?;
        if pos_size < 3 || pos_type_enum != VType::Float {
            return Err(RenderError::from(
                    &format!("bad position layout, got {} of type {:?}", pos_size, pos_type_enum)));
        }
        if col_size < 4 || col_type_enum != VType::Float {
            return Err(RenderError::from(
                    &format!("bad color layout, got {} of type {:?}", col_size, col_type_enum)));
        }
        if tuv_size < 2 || tuv_type_enum != VType::Float {
            return Err(RenderError::from(
                    &format!("bad tex uv layout, got {} of type {:?}", tuv_size, tuv_type_enum)));
        }
        if tid_type_enum != VType::Float {
            return Err(RenderError::from(
                    &format!("bad tex id layout, got {} of type {:?}", tuv_size, tuv_type_enum)));
        }

        let corners: [Vec3; 4] = [
            Vec3::new( self.whd.x,  self.whd.y, 0.0) / 2.0,
            Vec3::new(-self.whd.x,  self.whd.y, 0.0) / 2.0,
            Vec3::new(-self.whd.x, -self.whd.y, 0.0) / 2.0,
            Vec3::new( self.whd.x, -self.whd.y, 0.0) / 2.0,
        ];

        let translation = glam::Mat4::from_translation(self.pos);

        for mut corner in corners {
            // buffer vertex

            corner = (translation * Vec4::from((corner, 1.0))).truncate();

            // NOTE: it's ok to set a float to 0x00000000, that evaluates to 0.0
            for _ in 0..buf.layout_len() {
                buf.vb.push(0u8);
            }

            for i in 0..3 {
                buf.vb.set(
                    offset + (pos_pos + i) * pos_type_enum.size_bytes(),
                    corner[i])
                    .or(Err(RenderError::from(&format!("bad block insertion"))))?;
            }
            for i in 0..4 {
                buf.vb.set(
                    offset + (col_pos + i) * col_type_enum.size_bytes(),
                    self.color[i])
                    .or(Err(RenderError::from(&format!("bad block insertion"))))?;
            }
            for i in 0..2 {
                buf.vb.set(
                    offset + (tuv_pos + i) * tuv_type_enum.size_bytes(),
                    0.0)
                    .or(Err(RenderError::from(&format!("bad block insertion"))))?;
            }
            buf.vb.set(
                offset + tid_pos * tid_type_enum.size_bytes(),
                0)
                .or(Err(RenderError::from(&format!("bad block insertion"))))?;

            offset += buf.layout_len() as usize;
        }

        buf.size += 1;

        Ok(())
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    // starting window

    let mut glfw: glfw::Glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(680, 400, TITLE, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    window.set_key_polling(true);
    window.make_current();

    glfw.default_window_hints();
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    // gl
    gl::load_with(|s| window.get_proc_address(s) as * const _);

    renderer::start();


    let r: Rect = Rect{pos: vec3(0.0, 0.0, 0.0), whd: vec3(1.0, 1.0, 1.0), color: vec4(1.0, 1.0, 1.0, 1.0)};


    while !window.should_close() {
        unsafe { T1 = glfw.get_time(); }

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
        unsafe { r.to_buffer(&mut DEFAULT_VB)?; }

        renderer::update();


        window.swap_buffers();

        unsafe {
            T2 = glfw.get_time();
            DT = T2 - T1;
        };
    }

    renderer::stop();

    Ok(())
}

const TITLE: &str = "Quick Render Screen";

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        glfw::WindowEvent::MouseButton(button, action, _) => {
            mouse::mouse_button_event(button, action);
        }
        glfw::WindowEvent::CursorPos(x, y) => {
            mouse::mouse_pos_event(x, y);
        }
        glfw::WindowEvent::Size(x, y) => {
            window::set_width_height(x as usize, y as usize);
            unsafe {
                gl::Viewport(0, 0, x, y);
            }
        }
        _ => {}
    }
}
