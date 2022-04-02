extern crate glfw;
extern crate gl;
extern crate cgmath;

use gl::types::*;

use std::fs;

use glfw::{Action, Context, Key, WindowEvent};
use cgmath::prelude::*;
use cgmath::{vec2, perspective, Deg, Matrix4, vec3};
use ldjam50::engine;

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

fn main() {
    let (mut eng, mut event_handler) = engine::init();

    // TODO: move all the shader stuff to the engine mod
    let shader: u32;
    unsafe {
        shader = gl::CreateProgram();
        let vertex_shader = compile_shader(
            &read_shader("./data/shaders/base.vert"), 
            gl::VERTEX_SHADER
        );
        let fragment_shader = compile_shader(
            &read_shader("./data/shaders/base.frag"), 
            gl::FRAGMENT_SHADER
        );

        gl::AttachShader(shader, vertex_shader);
        gl::AttachShader(shader, fragment_shader);
        gl::LinkProgram(shader);
    }

    // TODO: find a better way for this 
    let mut rect1 = engine::Object::new("test.jpg", vec2(0.4, 0.4), shader);
    rect1.set_proj(perspective(Deg(45.), 1., 0.01, 100.));
    rect1.set_view(Matrix4::from_translation(vec3(0., 0., -3.)));

    #[allow(unused_labels)]
    'main_loop: while eng.is_working() {
        unsafe {
            gl::ClearColor(0.2, 0.1, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        
        // FIXME: make this work properly
        rect1.update();
        
        for (_, event) in event_handler.get() {
            if let WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                eng.stop_working();
            }
        }

        eng.tick();
    }
}