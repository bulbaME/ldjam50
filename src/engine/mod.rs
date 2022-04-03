extern crate gl;
extern crate glfw;
extern crate cgmath;

pub mod engine;
pub mod object;
pub mod init;
pub mod event_handler;
mod renderer;

pub use engine::Engine;
pub use object::Object;
pub use init::init;
pub use event_handler::EventHandler;

use glfw::*;

use renderer::Renderer;

pub type Position = (i32, i32, i32);
pub type Size = (u32, u32);

static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 800;
static WINDOW_TITLE: &str = "LD50 Game";

