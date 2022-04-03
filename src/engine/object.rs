use super::*;

use cgmath::prelude::*;
use cgmath::{Matrix4};

pub struct Object {
    sprite: renderer::Sprite,
    position: Position,
    size: Size
}

impl Object {
    pub fn new(img_path: &'static str, size: (u32, u32)) -> Object {
        Object {
            sprite: renderer::Sprite::new(img_path, gl::LINEAR as i32),
            position: (0, 0, -1),
            size: size
        }
    }

    pub fn update(&self) {

    }

    pub fn get_sprite(&self) -> &renderer::Sprite {
        &(self.sprite)
    }

    pub fn set_position(&mut self, pos: Position) {
        self.position = pos;
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    pub fn get_size(&self) -> Size {
        self.size
    }
}