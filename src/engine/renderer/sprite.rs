use super::*;

pub struct Sprite <'a> {
    mesh: Mesh,
    texture: Texture,
    size: Size,
    position: Position,
    shader: &'a Shader
}

impl <'a> Sprite <'a> {
    pub fn new<'b>(path: &str, filter: i32, shader: &'b Shader, mirrored: bool) -> Sprite<'b> {
        let texture = Texture::new(path, filter, mirrored);
        Sprite {
            mesh: Mesh::new(),
            size: texture.get_size().clone(),
            texture: texture,
            position: vec3(0.0, 0.0, -1.0),
            shader: shader
        }
    }

    pub fn get_texture(&self) -> &Texture {
        &(self.texture)
    }

    pub fn get_mesh(&self) -> &Mesh {
        &(self.mesh)
    }

    pub fn get_shader(&self) -> &Shader {
        self.shader
    }
}

impl<'a> Positioning for Sprite<'a> {
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