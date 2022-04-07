pub mod fire;
pub mod water;
pub mod damage;
pub mod powder;
pub mod fireex;

mod particle;
mod emitter;
mod render;

pub use water::Water;
pub use fire::Fire;
pub use damage::Damage;
pub use powder::Powder;
pub use fireex::Fireex;

use super::*;

use std::ops::Range;

use rand::Rng;

type Vec2 = cgmath::Vector2<f32>;
type Vec3 = cgmath::Vector3<f32>;
type Vec4 = cgmath::Vector4<f32>;

fn get_rand_vec2(range: &Range<Vec2>, rnd: &mut rand::rngs::ThreadRng) -> Vec2 {
    Vec2 {
        x: rnd.gen_range(range.start.x..range.end.x),
        y: rnd.gen_range(range.start.y..range.end.y),
    }
}

fn get_rand_vec4(range: &Range<Vec4>, rnd: &mut rand::rngs::ThreadRng) -> Vec4 {
    Vec4 {
        x: rnd.gen_range(range.start.x..range.end.x),
        y: rnd.gen_range(range.start.y..range.end.y),
        z: rnd.gen_range(range.start.z..range.end.z),
        w: rnd.gen_range(range.start.w..range.end.w),
    }
}

pub trait Particle {
    fn render(&mut self, engine: &mut Engine, vp: &Matrix4<f32>);
}