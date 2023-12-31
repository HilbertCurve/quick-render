use crate::sandbox::{
    buffer::VertexBuffer,
    camera::Camera,
    primitive,
    shader::Shader,
};

use std::error::Error;
use std::fmt::{self, Display};
use std::ptr;

pub fn gl_err_check(line: u32) {
    let err = unsafe { gl::GetError() };
    if err != 0 {
        panic!("error here at line: {}! {}", line, err.to_string())
    }
}
pub fn gl_err_clear() {
    while unsafe { gl::GetError() } != 0 {}
}

#[derive(Debug)]
pub struct RenderError {
    what: String,
}

impl Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.what)
    }
}

impl Error for RenderError {}

impl RenderError {
    pub fn from(message: &str) -> RenderError {
        RenderError { what: String::from(message) }
    }
}

// TODO: render targets; no more `pub static VertexBuffer`s
pub enum RenderTarget {
    Sprite,
    Model,
}

pub trait Renderable {
    fn to_buffer(&self, buf: &mut VertexBuffer) -> Result<(), RenderError>;
}

static mut DEFAULT_SHADER: Shader = Shader::new_uninit();

const VERT_CODE: &str = 
"#version 330 core
#ifdef GL_ES
 precision mediump float;
#endif

layout (location=0) in vec3 aPos;
layout (location=1) in vec4 aColor;
layout (location=2) in vec2 aTexUV;
layout (location=3) in float aTexID;

uniform mat4 uProjection;
uniform mat4 uView;

out vec4 fPos;
out vec4 fColor;
out vec2 fTexUV;
out float fTexID;

void main()
{
    fPos = uProjection * uView * vec4(aPos, 1.0);
    fColor = aColor;
    fTexUV = aTexUV;
    fTexID = aTexID;
    gl_Position = uProjection * uView * vec4(aPos, 1.0);
}";
const FRAG_CODE: &str = 
"#version 330 core

in vec4 fPos;
in vec4 fColor;
in vec2 fTexUV;
in float fTexID;
// uniform float uTime;

out vec4 color;
void main()                          
{
    color = fColor;
}";

pub static mut DEFAULT_VB: VertexBuffer = VertexBuffer::new();

pub fn start() {
    unsafe {
        // this spot is for initializing default vertex buf, data buf,
        // and shader, along with some gl settings

        DEFAULT_VB.set_layout(&VertexBuffer::DEFAULT_ATTRIBS);
        DEFAULT_VB.set_primitive(&primitive::QUAD);
        DEFAULT_VB.init(&[0.0; 0], &[0; 0]);
        DEFAULT_VB.bind();
        DEFAULT_VB.refresh();
        DEFAULT_SHADER = Shader::new(VERT_CODE.to_owned(), FRAG_CODE.to_owned());
        DEFAULT_VB.enable_attribs();

        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }
}

pub fn update() {
    // TODO: static vec of data buffers, render each according to their primitive
    unsafe {
        gl_err_clear();
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl_err_check(line!());

        DEFAULT_VB.bind();
        gl_err_check(line!());
        DEFAULT_VB.refresh();
        gl_err_check(line!());

        // attach textures
        /*
        for tex in TEX_POOL.try_lock().unwrap().iter() {
            gl::ActiveTexture(gl::TEXTURE0 + tex.get_id());
            gl::BindTexture(gl::TEXTURE_2D, tex.get_id());
        }
        */

        // shader stuff
        DEFAULT_SHADER.attach();
        gl_err_check(line!());
        DEFAULT_SHADER.set_uniform_mat4("uProjection", Camera::get().projection_mat());
        gl_err_check(line!());
        DEFAULT_SHADER.set_uniform_mat4("uView", Camera::get().view_mat());
        gl_err_check(line!());
        // there must be a better way to do this
        /*
        let mut ids;
        {
            let tpl = TEX_POOL.try_lock().unwrap();
            ids = vec![0i32;tpl.len()];
            let mut i = 0;
            for id in ids.iter_mut() {
                *id = tpl.get(i).unwrap().get_id() as i32;
                i += 1;
            }
        }
        */
        // make sure to attach integer values to the uTextures as well!!!
        /*
        DEFAULT_SHADER.set_uniform_i32_array(
            "uTextures",
            TEX_POOL.try_lock().unwrap().len() as i32,
            ids.as_ptr(),
        );
        */

        // vertex attrib pointers
        DEFAULT_VB.enable_attribs();
        gl_err_check(line!());

        // draw
        let p = DEFAULT_VB.prim;
        gl::DrawElements(
            p.gl_prim,
            DEFAULT_VB.ib.len() as i32 / std::mem::size_of::<u32>() as i32,
            gl::UNSIGNED_INT,
            ptr::null()
            );
        gl_err_check(line!());

        DEFAULT_VB.disable_attribs();
        gl_err_check(line!());

        DEFAULT_VB.unbind();
        gl_err_check(line!());
        DEFAULT_VB.clear();
        gl_err_check(line!());
        DEFAULT_SHADER.detach();
        gl_err_check(line!());

        // detach textures
        /*
        for tex in TEX_POOL.try_lock().unwrap().iter() {
            gl::ActiveTexture(gl::TEXTURE0 + tex.get_id());
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        */
    }
}

pub fn stop() {

}
