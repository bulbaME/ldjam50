extern crate gl;
extern crate glfw;
extern crate cgmath;
extern crate rand;

pub mod engine;
pub mod object;
pub mod init;
pub mod camera;
pub mod event_handler;
pub mod movement;
pub mod renderer;
pub mod collisions;
pub mod positioning;

pub use engine::Engine;
pub use object::Object;
pub use camera::Camera;
pub use renderer::Shader;
pub use init::init;
pub use event_handler::EventHandler;
pub use movement::Movement;
pub use renderer::Sprite;
pub use renderer::Text;
pub use renderer::Particle;
pub use collisions::Collision;
pub use positioning::Positioning;

use glfw::*;

use cgmath::{Vector3, Vector2, Matrix4, vec3, vec2, Vector4};

use renderer::Renderer;
 
pub type Position = Vector3<f32>;
pub type Size = Vector2<f32>;

static NO_SIZE: Vector2<f32> = Vector2 {x: 0.0, y: 0.0};
#[allow(dead_code)]
static NO_POS: Vector3<f32> = Vector3 {x: 0.0, y: 0.0, z: 0.0};

pub static WINDOW_WIDTH: u32 = 1200;
pub static WINDOW_HEIGHT: u32 = 800;
pub static WINDOW_TITLE: &str = "LD50 Game";

