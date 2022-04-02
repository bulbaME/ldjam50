extern crate gl;
extern crate glfw;
extern crate cgmath;
extern crate cstr;

pub mod renderer;

use cstr::cstr;
use glfw::{Action, Context, Key, WindowEvent, Glfw};
use glfw::ffi::glfwPollEvents;
use gl::types::*;

use std::sync::mpsc::Receiver;

use cgmath::prelude::*;
use cgmath::{Vector2, Matrix4, vec3, vec2};

static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;
static WINDOW_TITLE: &str = "LD50 Game";

#[cfg(target_os = "macos")]
unsafe fn set_window_hints() {
    use glfw::ffi::glfwWindowHint;

    let GLFW_OPENGL_FORWARD_COMPAT = 0x00022006;
    let GLFW_CONTEXT_VERSION_MAJOR = 0x00022002;
    let GLFW_CONTEXT_VERSION_MINOR = 0x00022003;
    let GLFW_OPENGL_PROFILE = 0x00022008;
    let GLFW_OPENGL_CORE_PROFILE = 0x00032001;

    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
    glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, 1);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
}

#[cfg(target_os = "windows")]
unsafe fn set_window_hints() {
    glfw::WindowHint::ContextVersion(3, 3);
    glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core);
}

pub fn init() -> (glfw::Window, Receiver<(f64, WindowEvent)>, Glfw) {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)
        .expect("Failed to initialize glfw");

   unsafe {
        set_window_hints();
    }

    let (mut window, events) = glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE, glfw::WindowMode::Windowed)
        .expect("Failed to create window");

    window.make_current();
    window.set_key_polling(true);
    
    // Load OpenGL functions
    gl::load_with(|symb| window.get_proc_address(symb));

    // Return the Engine
    (window, events, glfw)
}


/*
pub fn init() -> (Engine, EventHandler) {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)
        .expect("Failed to initialize glfw");

   unsafe {
        set_window_hints();
    }

    let (mut window, events) = glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE, glfw::WindowMode::Windowed)
        .expect("Failed to create window");

    window.make_current();
    window.set_key_polling(true);
    
    // Load OpenGL functions
    gl::load_with(|symb| window.get_proc_address(symb));

    // Return the Engine
    (Engine {
        window: window,
        objects: vec![],
        camera: Vector2 {x: 0., y: 0.}
    }, EventHandler {
        events: events,
        glfw: glfw
    })
}
*/
pub struct EventHandler {
    events: Receiver<(f64, WindowEvent)>,
    glfw: glfw::Glfw
}

/*
pub struct EventHandler {
    events: Receiver<(f64, WindowEvent)>,
    glfw: glfw::Glfw
}

impl EventHandler {
    fn pull(&mut self) {
        self.glfw.poll_events();
    }

    pub fn get(&mut self) -> glfw::FlushedMessages<'_, (f64, WindowEvent)> {
        self.pull();
        glfw::flush_messages(&self.events)
    }
}

pub struct Engine {
    window: glfw::Window,
    objects: Vec<Object>,
    
    camera: Vector2<f32>
}

impl Engine {

    pub fn tick(&mut self) {
        self.window.swap_buffers();
    }

    // engine state
    pub fn is_working(&self) -> bool {
        !self.window.should_close()
    }

    pub fn stop_working(&mut self) {
        self.window.set_should_close(true);
    }
}

pub struct Object {
    sprite: Sprite,
    view: Matrix4<f32>,
    projection: Matrix4<f32>,
    shader: u32
}

impl Object {
    pub fn new(img_path: &'static str, size: Vector2<f32>, shader: u32) -> Object {
        Object {
            sprite: Sprite::new(img_path, size),
            view: Matrix4::<f32>::identity(),
            projection: Matrix4::<f32>::identity(),
            shader
        }
    }

    pub fn update(&self) {
        let vp = self.projection * self.view;

        unsafe {
            self.sprite.draw(vp, self.shader);
        }
    }

    pub fn set_shader(&mut self, shader: u32) {
        self.shader = shader;
    }

    pub fn set_view(&mut self, view: Matrix4<f32>) {
        self.view = view;
    }

    pub fn set_proj(&mut self, proj: Matrix4<f32>) {
        self.projection = proj;
    }
}

pub struct Sprite {
    vao: u32,
    vbo: u32,
    texture: u32,
    size: Vector2<f32>,
    position: Vector2<f32>,
}

use image::io::Reader as ImageReader;
impl Sprite {

    pub fn new(img_path: &'static str, size: Vector2<f32>) -> Sprite {
        let (vao, vbo) = unsafe {
            renderer2::init_rect(size)
        };
        
        // texture
        let img = ImageReader::open(["data/textures/", img_path].concat()).expect("Failed to open image")
            .decode().expect("Failed to decode image")
            .into_rgb8();
        let img = image::imageops::flip_vertical(&img);
        let (t_width, t_height) = img.dimensions();
        let img_data = img.into_raw();

        let mut texture = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D, 0, 
                gl::RGB as i32, t_width as i32, t_height as i32, 
                0, gl::RGB, gl::UNSIGNED_BYTE,
                img_data.as_ptr().cast()
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Sprite {
            vao: vao,
            vbo: vbo,
            texture: texture,
            size: size,
            position: Vector2 {x: 0., y: 0.},
        }
    }

    pub unsafe fn draw(&self, vp: Matrix4<f32>, shader: u32) {
        gl::UseProgram(shader);
        gl::BindTexture(gl::TEXTURE_2D, self.texture);
        gl::BindVertexArray(self.vao);

        let mut model = Matrix4::<f32>::identity();
        model = model * Matrix4::from_translation(vec3(self.position.x, self.position.y, 1.));
        
        // MVP matrix
        let mvp = vp * model;
        let mvp_loc = gl::GetUniformLocation(shader, cstr!("uMVP").as_ptr());
        gl::UniformMatrix4fv(mvp_loc, 1, gl::FALSE, mvp.as_ptr());

        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);
        gl::BindVertexArray(0);
    }
} */