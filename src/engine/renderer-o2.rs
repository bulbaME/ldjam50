extern crate cgmath;
extern crate gl;

use gl::types::*;
use image::io::Reader as ImageReader;
use std::fs;
use std::mem;

use cgmath::{Matrix4, Deg, Rad, perspective, Vector2};
use cgmath::prelude::*;

fn load_texture(path: &'static str) -> image::RgbImage {
    let img = ImageReader::open(path).unwrap().decode().unwrap().into_rgb8();
    let img = image::imageops::flip_vertical(&img);
    img
}

type Vert = [f32; 2];

/* 
    I decided to reimplement renderer.rs 
    because it uses non-rust things.
    Make this thing work, but using
    SAFE RUST! thx.
*/

pub unsafe fn init_rect(size: Vector2<f32>) -> (u32, u32) {
    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;

    let indicies: [u32; 6] = [
        0, 1, 3,
        1, 2, 3
    ];

    let buffer: [Vert; 4] = [
        [0.,     size.y], // top right
        [size.x, size.y], // bottom right
        [size.x,     0.], // bottom left
        [0.,         0.]  // top left
    ];

    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(1, &mut vbo);
    gl::GenBuffers(1, &mut ebo);

    gl::BindVertexArray(vao);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        mem::size_of_val(&buffer) as isize, 
        buffer.as_ptr().cast(),
        gl::DYNAMIC_DRAW
    ); 

    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        mem::size_of_val(&indicies) as isize,
        indicies.as_ptr().cast(),
        gl::STATIC_DRAW
    );

    // position
    gl::VertexAttribPointer(
        0,  // (location = 0)
        2,  // vec2
        gl::FLOAT, gl::FALSE,
        (8 * mem::size_of::<f32>()) as i32, 
        0 as *const _
    );
    gl::EnableVertexAttribArray(0);

    gl::BindVertexArray(0);
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

    (vao, vbo)
}

pub unsafe fn bind_rect() {

}