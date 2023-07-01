pub mod buffer;
pub mod camera;
pub mod mouse;
pub mod primitive;
pub mod renderer;
pub mod shader;
pub mod window;

pub mod prelude {
    pub use super::renderer::{self, Renderable, RenderError};
    pub use super::camera::Camera;
    pub use super::buffer::*;
    pub use super::mouse;
    pub use super::window::{self, DEFAULT_WH};
}