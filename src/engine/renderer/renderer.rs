use super::*;

use cgmath::prelude::*;
use cgmath::{Matrix4, vec3};

pub struct Renderer {}

impl Renderer {
    // pub fn new() -> Renderer {
    //     // TODO: Default texture support
    //     unsafe {
    //         gl::Viewport(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);            
    //     }

    //     Renderer {
    //         shader: Shader::new("base.vert", "base.frag")
    //     }
    // }

    pub fn draw_quad(shader: &Shader, sprite: &Sprite, position: Position, size: Size, view: Matrix4<f32>) {
        let position = vec3(
            position.0 as f32, 
            position.1 as f32, 
            position.2 as f32
        );

        let texture = sprite.get_texture();
        let mesh = sprite.get_mesh();

        // Bind the texture
        texture.bind(0);

        // Calculate the MVP
        let proj: Matrix4<f32> = cgmath::ortho(0.0, WINDOW_WIDTH as f32, 0.0, WINDOW_HEIGHT as f32, 0.0, 100.0);
        let mut model = Matrix4::<f32>::identity();
        model = model * Matrix4::from_translation(position);
        // model = model * Matrix4::<f32>::from_scale(size.x);
        model[0][0] *= size.0 as f32;  // scale x 
        model[1][1] *= size.1 as f32;  // scale y


        let mvp: Matrix4<f32> = proj * view * model;

        // Set uniforms 
        shader.set_int("uTexture", 0); 
        shader.set_mat4("uMVP", &mvp); 
        
        unsafe {
            // Bind the mesh
            gl::BindVertexArray(mesh.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ebo); // it is unnecessary to have it here

            // Draw the quad
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);

            // Unbind the mesh
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }

        // Unbind the texture
        texture.unbind();
    }
}