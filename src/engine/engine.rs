use super::*;

use renderer::renderer::Renderer;

use cgmath::{Matrix4, vec3};

pub fn init<'a>(window: Window) -> Engine<'a> {
    Engine {
        window,
        objects: vec![],
        camera: (0, 0, 0),
        shader: renderer::Shader::new("base.vert", "base.frag")
    }
}

pub struct Engine <'a> {
    window: Window,
    objects: Vec<&'a Object>,
    camera: Position,
    shader: renderer::Shader
}

impl<'a> Engine <'a> {
    pub fn bind_object(&mut self, object: &'a Object) {
        self.objects.push(object);
    }

    pub fn tick(&mut self) {
        self.update();
        self.render();
    }

    fn render(&mut self) {
        let view = Matrix4::<f32>::from_translation(vec3(
            self.camera.0 as f32,
            self.camera.1 as f32,
            self.camera.2 as f32
        ));

        unsafe {
            gl::ClearColor(0.12, 0.1, 0.24, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.shader.bind();

        for o in &self.objects {
            Renderer::draw_quad(
                &(self.shader), 
                o.get_sprite(), 
                o.get_position(), 
                o.get_size(), 
                view
            );
        }

        self.window.swap_buffers();
    }

    fn update(&self) {

    }

    // engine state
    pub fn is_working(&self) -> bool {
        !self.window.should_close()
    }

    pub fn stop_working(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn set_shader(&mut self, shader: renderer::Shader) {
        self.shader = shader;
    }


    // camera movement
    pub fn move_camera(&mut self, pos: Position) {
        self.move_camera_x(pos.0);
        self.move_camera_y(pos.1);
        self.move_camera_z(pos.2);
    }

    pub fn move_camera_x(&mut self, pos: i32) {
        self.camera.0 += pos;
    }

    pub fn move_camera_y(&mut self, pos: i32) {
        self.camera.1 += pos;
    }

    pub fn move_camera_z(&mut self, pos: i32) {
        self.camera.2 += pos;
    }

    pub fn set_camera(&mut self, pos: Position) {
        self.camera = pos;
    }

    pub fn set_camera_x(&mut self, pos: i32) {
        self.camera.0 = pos;
    }

    pub fn set_camera_y(&mut self, pos: i32) {
        self.camera.1 = pos;
    }

    pub fn set_camera_z(&mut self, pos: i32) {
        self.camera.2 = pos;
    }
}