extern crate gl;
extern crate glfw;

use glfw::{Action, Context, Key};
use gl::types::*;

use cgmath::{Matrix4, vec3, Deg, Rad, perspective};
use cgmath::prelude::*;

use std::mem;

static mut vao_: u32;
static mut vbo_: u32;
static mut ebo_: u32;

static vertex_buffer_: [f32; 12] = [
    0.0, 0.0, 1.0,
    1.0, 0.0, 1.0,
    1.0, 1.0, 1.0,
    0.0, 1.0, 1.0
];
static vertex_size: usize = 3 * mem::size_of::<f32>(); 

static index_buffer_: [u32; 6] = [
    0, 1, 2,
    2, 3, 0
];

fn read_shader(name: &'static str) -> String {
    fs::read_to_string(name)
        .expect("Cannot read the shader")
}

unsafe fn compile_shader(code: &str, shader_type: GLenum) -> u32 {
    let shader = gl::CreateShader(shader_type);
    gl::ShaderSource(
        shader, 1, 
        &(code.as_bytes().as_ptr().cast()),
        &(code.len().try_into().unwrap())
    );
    gl::CompileShader(shader);
    let mut result = 0;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut result);
    if result == 0 {
        let mut log: Vec<u8> = Vec::with_capacity(1024);
        let mut log_len = 0i32;
        gl::GetShaderInfoLog(
            shader,
            1024,
            &mut log_len,
            log.as_mut_ptr().cast()
        );
        log.set_len(log_len.try_into().unwrap());
        panic!("Shader compile error: {}", String::from_utf8_lossy(&log));
    }

    shader
}

// 0 means no shader is bound becase 0 is not a valid shader id.
static mut bound_shader: u32 = 0;

unsafe fn set_bound_shader(id: u32) {
    bound_shader = id;
}

unsafe fn get_bound_shader() -> u32 {
    bound_shader
}


pub struct Shader {
    id: u32
}

impl Shader {
    pub fn new(vertex: &str, fragment: &str) -> Shader {
        Shader {
            id: compile_shader(vertex, fragment),
        }
    }

    pub fn get_bound_shader() -> Shader {
        Shader {
            id: get_bound_shader()
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
            set_bound_shader(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
            set_bound_shader(0);
        }
    }
}

pub fn init_renderer() { 

    // Create VAO
    gl::GenVertexArrays(1, &mut vao_);
    gl::BindVertexArray(vao_);

    // Create VBO and send initial data, this data on the GPU won't be changed.
    gl::GenBuffers(1, &mut vbo_);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo_);
    gl::BufferData(
        gl::ARRAY_BUFFER, 
        mem::size_of_val(&vertex_buffer_) as isize,
        vertex_buffer_.as_ptr().cast(),
        gl::STATIC_DRAW
    );

    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, vertex_size, ptr::null());
    gl::EnableVertexAttribArray(0);

    // Create EBO and send initial data, this data on the GPU won't be changed.
    gl::GenBuffers(1, &mut ebo_);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo_);
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

    // Enable blending
    gl::Enable(gl::BLEND);
    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
}

pub fn draw_quad(position: Vector3, size: Vector2, color: Vector4) {
    panic!("ERROR: `draw_quad` not implemented yet.");    

    let bound_shader = Shader::get_bound_shader();

    // Bind texture here ...

    proj = Matrix4::<f32>::identity(); // Temporary, get this from camera
    view = Matrix4::<f32>::identity(); // Temporary, get this from camera
    let mut model = Matrix4::<f32>::identity();
    model = model * Matrix4::from_translation(position);
    model = model * Matrix4::from_scale(size.x);

    let mvp = proj * view * model;

    // Set uniforms here ...

    gl::BindVertexArray(vao_);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo_);

    gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, mem::null());

    // Unbind texture here ...
}