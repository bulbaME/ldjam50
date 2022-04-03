use super::*;

pub struct Texture {
    texture_id: u32,
    size: Size
}

impl Texture {
    pub fn new(path: &str, filter: i32) -> Texture {

        use image::io::Reader as ImageReader;

        let mut texture: u32 = 0;
        let (width, height): (u32, u32);

        let img = ImageReader::open(["data/textures/", path].concat()).expect("Failed to open image")
            .decode().expect("Failed to decode image")
            .into_rgba8();
        let img = image::imageops::flip_vertical(&img);

        (width, height) = img.dimensions();
        let data = img.into_raw();
        
        unsafe {
            gl::GenTextures(1, &mut texture);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            let mipmap_filter = if filter == gl::LINEAR as i32 {
                gl::LINEAR_MIPMAP_LINEAR as i32 
            } else { 
                gl::NEAREST_MIPMAP_NEAREST as i32 
            };

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, mipmap_filter);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filter);

            gl::TexImage2D(
                gl::TEXTURE_2D, 0, 
                gl::RGBA8 as i32,
                width as i32, height as i32, 
                0, gl::RGBA, 
                gl::UNSIGNED_BYTE,
                data.as_ptr().cast()
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        // Return the texture
        Texture {
            texture_id: texture,
            size: vec2(width as f32, height as f32)
        }
    }

    pub fn bind(&self, slot: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + slot);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn get_size(&self) -> &Size {
        &(self.size)
    }
}