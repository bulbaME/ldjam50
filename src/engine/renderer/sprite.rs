use super::mesh::Mesh;
use super::texture::Texture;

pub struct Sprite {
    mesh: Mesh,
    texture: Texture,
    size: (u32, u32)
}

impl Sprite {
    pub fn new(path: &str, filter: i32) -> Sprite {
        let texture = Texture::new(path, filter);
        Sprite {
            mesh: Mesh::new(),
            size: texture.get_size(),
            texture: texture,
        }
    }

    pub fn get_texture(&self) -> &Texture {
        &(self.texture)
    }

    pub fn get_mesh(&self) -> &Mesh {
        &(self.mesh)
    }
}