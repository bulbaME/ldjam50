use super::*;

pub enum Object <'a> {
    Sprite(&'a Sprite<'a>),
    Text(&'a Text<'a>),
    Particle(&'a mut dyn Particle)
}