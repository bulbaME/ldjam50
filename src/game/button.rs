use super::*;

pub struct Button <'a> {
    pub unfocused: Sprite<'a>,
    pub focused: Sprite<'a>,
    position: Position,
    size: Size
}

impl <'a> Button <'a> {
    pub fn new(t1: &str, t2: &str, shader: &'a engine::renderer::shader::Shader) -> Button<'a> {
        let s1 = Sprite::new(t1, gl::NEAREST as i32, shader);
        let s2 = Sprite::new(t2, gl::NEAREST as i32, shader);
        Button {
            size: s1.get_size().clone(),
            unfocused: s1, 
            focused: s2, 
            position: vec3(0.0, 0.0, -1.0),
        }
    }
}

impl <'a> Positioning for Button <'a> {
    fn get_position(&self) -> &Position {
        &(self.position)
    }

    fn get_size(&self) -> &Size {
        &(self.size)
    }

    fn set_position(&mut self, pos: &Position) {
        self.position = pos.clone();
        self.unfocused.set_position(pos);
        self.focused.set_position(pos);
    }

    fn set_size(&mut self, size: &Size) {
        self.size = size.clone();
        self.unfocused.set_size(size);
        self.focused.set_size(size);
    }
}