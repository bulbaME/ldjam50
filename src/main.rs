extern crate glfw;
extern crate gl;
extern crate cgmath;

use gl::types::*;

use std::fs;

use glfw::{Action, Context, Key, WindowEvent};
use cgmath::prelude::*;
use cgmath::{vec2, perspective, Deg, Matrix4, vec3};
use ldjam50::engine;
use ldjam50::engine::renderer;

fn main() {
    let (mut window, event_handler, mut glfw) = engine::init();

    /* 
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
    */

    let rend = renderer::Renderer::new();

    let shader = renderer::Shader::new("./data/shaders/base.vert", "./data/shaders/base.frag");
    
    let sprite = renderer::Sprite::new("test.jpg", gl::LINEAR as i32);
    

    while !window.should_close() {

        unsafe {
            gl::ClearColor(0.12, 0.1, 0.24, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        shader.bind();

        rend.draw_quad(
            &shader, 
            &sprite, 
            &cgmath::vec3::<f32>(200.0, 200.0, -1.0), 
            &cgmath::vec2::<f32>(200.0, 250.0)
        );

        window.swap_buffers();
        glfw.poll_events();
    }
}