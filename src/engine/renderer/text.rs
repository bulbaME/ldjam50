use super::*;

pub struct Text <'a> {
    mesh: Mesh,
    size: Size,
    font: Texture,
    position: Position,
    shader: &'a Shader,
    text: String
}

impl <'a> Text <'a> {
    pub fn new<'b>(font: &str, text: &str, shader: &'b Shader) -> Text<'b> {
        Text {
            mesh: Mesh::new(),
            size: vec2(1.0, 1.0),
            font: Texture::new(font, gl::NEAREST as i32),
            position: vec3(0.0, 0.0, -1.0),
            shader: shader,
            text: text.to_string()
        }
    }

    pub fn get_text(&self) -> &str {
        &(self.text)
    }

    pub fn get_font(&self) -> &Texture {
        &(self.font)
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn get_mesh(&self) -> &Mesh {
        &(self.mesh)
    }

    pub fn get_shader(&self) -> &Shader {
        self.shader
    }
}

impl<'a> Positioning for Text<'a> {
    fn get_position(&self) -> &Position {
        &(self.position)
    }

    fn get_size(&self) -> &Size {
        &(self.size)
    }

    fn set_position(&mut self, pos: &Position) {
        self.position = pos.clone();
    }

    fn set_size(&mut self, size: &Size) {
        self.size = size.clone();
    }
}