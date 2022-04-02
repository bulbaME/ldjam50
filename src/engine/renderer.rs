#[allow(unused_imports)]
extern crate glfw;
extern crate cgmath;

extern crate gl;
use self::gl::types::*;

use std::mem;
use std::ffi::CString;

use image;

use cgmath::prelude::*;
use cgmath::{Matrix4, Vector3, Vector2};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn read_shader(path: &str) -> String {
    std::fs::read_to_string(path)
        .expect("Cannot read the shader")
}

unsafe fn compile_shader(code: &str, shader_type: GLenum) -> u32 {
    let shader = gl::CreateShader(shader_type);
    gl::ShaderSource(
        shader, 1, 
        &(code.as_bytes().as_ptr().cast()),
        std::ptr::null()
    );
    gl::CompileShader(shader);

    let mut lenght: i32 = 512;
    let mut info_log: Vec<u8> = Vec::with_capacity(lenght as usize);
    info_log.set_len(lenght as usize - 1);

    let mut result = gl::FALSE as i32;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut result);
    if result != gl::TRUE as i32 {
        gl::GetShaderInfoLog(
            shader,
            512,
            &mut lenght,
            info_log.as_mut_ptr().cast()
        );
        println!("Shader compile error: {}", String::from_utf8_lossy(&info_log));
    }

    shader
}

unsafe fn create_shader_program(vertex: &str, fragment: &str) -> u32 {
    let shader: u32 = gl::CreateProgram();

    let vertex_shader = compile_shader(
        &read_shader(vertex), 
        gl::VERTEX_SHADER
    );
    let fragment_shader = compile_shader(
        &read_shader(fragment), 
        gl::FRAGMENT_SHADER
    );

    gl::AttachShader(shader, vertex_shader);
    gl::AttachShader(shader, fragment_shader);
    gl::LinkProgram(shader);

    let mut info_log: Vec<u8> = Vec::with_capacity(512);
    info_log.set_len(512 - 1);

    let mut success = gl::FALSE as i32;
    gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
    if success != gl::TRUE as i32 {
        gl::GetProgramInfoLog(shader, 512, core::ptr::null_mut(), info_log.as_mut_ptr().cast());
        println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&info_log));
    }

    return shader;
}

pub struct Shader {
    id: u32
}

impl Shader {
    pub fn new(vertex: &str, fragment: &str) -> Shader {
        Shader {
            id: unsafe {create_shader_program(vertex, fragment)},
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn set_int(&self, location: &'static str, value: i32) {
        unsafe {
            let location_ = CString::new(location).unwrap();
            let loc = gl::GetUniformLocation(self.id, location_.as_ptr());

            if loc == -1 {
                panic!("WARNING: `{}` is not a valid uniform location!", location);
            }

            gl::Uniform1i(loc, value);
        }
    }

    pub fn set_mat4(&self, location: &str, mat4: &Matrix4<f32>) {
        unsafe {
            let location_ = CString::new(location).unwrap();
            let loc = gl::GetUniformLocation(self.id, location_.as_ptr());

            if loc == -1 {
                panic!("WARNING: `{}` is not a valid uniform location!", location);
            }

            gl::UniformMatrix4fv(loc, 1, gl::FALSE, mat4.as_ptr())
        }
    }
}

pub struct Texture {
    texture_id: u32,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn new(path: &str, filter: i32) -> Texture {

        use image::io::Reader as ImageReader;

        let mut texture = Texture { texture_id: 0, width: 0, height: 0 };

        let img = ImageReader::open(["data/textures/", path].concat()).expect("Failed to open image")
            .decode().expect("Failed to decode image")
            .into_rgb8();
        let img = image::imageops::flip_vertical(&img);

        (texture.width, texture.height) = img.dimensions();
        let data = img.into_raw();

        unsafe {
            gl::GenTextures(1, &mut texture.texture_id);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture.texture_id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            let mipmap_filter = if filter == gl::LINEAR as i32 {
                gl::LINEAR_MIPMAP_LINEAR as i32 
            } else { 
                gl::NEAREST_MIPMAP_NEAREST as i32 
            };

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, mipmap_filter);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filter);

            gl::TexImage2D(
                gl::TEXTURE_2D, 0, 
                gl::RGB as i32,
                texture.width as i32, texture.height as i32, 
                0, gl::RGB, 
                gl::UNSIGNED_BYTE,
                data.as_ptr().cast()
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        // Return the texture
        texture
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
}

pub struct Mesh {
    vao: u32,
    vbo: u32,
    ebo: u32,
}

impl Mesh {
    pub fn new() -> Mesh {

        // Vertex data
        let vertex_buffer_: [f32; 12] = [
            0.0, 0.0, 1.0,
            1.0, 0.0, 1.0,
            1.0, 1.0, 1.0,
            0.0, 1.0, 1.0
        ];
        let vertex_size: usize = 3 * mem::size_of::<f32>(); 

        // Index data
        let index_buffer_: [u32; 6] = [
            0, 1, 2,
            2, 3, 0
        ];

        // Mesh
        let mut mesh = Mesh{ vao: 0, vbo: 0, ebo: 0 };

        unsafe {
            // Create VAO
            gl::GenVertexArrays(1, &mut mesh.vao);
            gl::BindVertexArray(mesh.vao);

            // Create VBO and send initial data, this data on the GPU won't be changed.
            gl::GenBuffers(1, &mut mesh.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                mem::size_of_val(&vertex_buffer_) as isize,
                vertex_buffer_.as_ptr().cast(),
                gl::STATIC_DRAW
            );

            gl::VertexAttribPointer(0, 3,
                gl::FLOAT, gl::FALSE,
                vertex_size as i32, core::ptr::null()
            );
            gl::EnableVertexAttribArray(0);

            // Create EBO and send initial data, this data on the GPU won't be changed.
            gl::GenBuffers(1, &mut mesh.ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER, 
                mem::size_of_val(&index_buffer_) as isize,
                index_buffer_.as_ptr().cast(),
                gl::STATIC_DRAW
            );

            // Unbind Objects
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        // Return mesh
        mesh
    }
}

pub struct Sprite {
    mesh: Mesh,
    texture: Texture,
}

impl Sprite {
    pub fn new(path: &str, filter: i32) -> Sprite {
        Sprite {
            mesh: Mesh::new(),
            texture: Texture::new(path, filter),
        }
    }
}

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Renderer {
        // TODO: Default texture support
        Renderer {}
    }

    pub fn draw_quad(&self, shader: &Shader, sprite: &Sprite, position: &Vector3<f32>, size: &Vector2<f32>) {

        // Bind the texture
        sprite.texture.bind(0);

        // Calculate the MVP
        let proj: Matrix4<f32> = cgmath::ortho(0.0, SCREEN_WIDTH as f32, 0.0, SCREEN_HEIGHT as f32, 0.0, 100.0);        
        let mut model: Matrix4<f32> = Matrix4::identity();
        model = model * Matrix4::<f32>::from_translation(position.clone());
        model = model * Matrix4::<f32>::from_scale(size.x); 

        let mvp: Matrix4<f32> = proj * model;

        // Set uniforms
        shader.set_int("uTexture", sprite.texture.texture_id as i32);
        shader.set_mat4("uMVP", &mvp);

        unsafe {
            // Bind the mesh
            gl::BindVertexArray(sprite.mesh.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, sprite.mesh.ebo);

            // Draw the quad
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, core::ptr::null());

            // Unbind the mesh
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }

        // Unbind the texture
        sprite.texture.unbind();
    }
}

