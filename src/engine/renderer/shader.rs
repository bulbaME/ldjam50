use gl::types::*;
use std::ffi::CString;

use super::*;

fn read_shader(path: &str) -> String {
    std::fs::read_to_string(["data/shaders/", path].concat())
        .expect("Cannot read the shader")
}

unsafe fn compile_shader(code: &str, shader_type: GLenum) -> u32 {

    let shader = gl::CreateShader(shader_type);
    gl::ShaderSource(
        shader, 1, 
        &(code.as_bytes().as_ptr().cast()), 
        &(code.len().try_into().unwrap()),
    );
    gl::CompileShader(shader);

    let mut lenght: i32 = 512;
    let mut info_log: Vec<u8> = Vec::with_capacity(lenght as usize);
    info_log.set_len(lenght as usize - 1);

    let mut result = gl::FALSE as i32; 
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut result);
    if result == gl::FALSE as i32 {
        gl::GetShaderInfoLog(
            shader,
            512,
            &mut lenght,
            info_log.as_mut_ptr().cast()
        );
        panic!("Shader compile error: {}", String::from_utf8_lossy(&info_log));
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
    if success == gl::FALSE as i32 {
        gl::GetProgramInfoLog(shader, 512, core::ptr::null_mut(), info_log.as_mut_ptr().cast());
        println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&info_log));
    }

    shader
}

pub enum SetType <'a> {
    Int(i32),
    Mat4(&'a Matrix4<f32>),
    Vec4(&'a Vector4<f32>)
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

    pub fn set(&self, location: &'static str, value: SetType) {
        unsafe {
            let location_ = CString::new(location).unwrap();
            let loc = gl::GetUniformLocation(self.id, location_.as_ptr());

            if loc == -1 {
                println!("WARNING: `{}` is not a valid uniform location!", location);
            }

            match value {
                SetType::Int(v) => gl::Uniform1i(loc, v),
                SetType::Mat4(v) => gl::UniformMatrix4fv(loc, 1, gl::FALSE, v.as_ptr()),
                SetType::Vec4(v) => gl::Uniform4fv(loc, 1, v.as_ptr())
            }
        }
    }
}
