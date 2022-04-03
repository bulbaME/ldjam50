use super::*;

use cgmath::prelude::*;
use cgmath::{Matrix4};

pub enum Object <'a> {
    Sprite(Sprite<'a>),
    Text(Text<'a>),
    Particle(Particle)
}