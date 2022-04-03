use super::*;
use std::time::SystemTime;

pub fn init(window: Window) -> Engine {
    Engine {
        window: window,
        time: SystemTime::now()
    }
}

pub struct Engine {
    window: Window,
    time: SystemTime
}

impl Engine {
    pub fn pre_render(&mut self) {
        self.window.swap_buffers();

        unsafe {
            gl::ClearColor(0.12, 0.1, 0.24, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn render_object(&self, object: &Object, vp: &Matrix4<f32>) {
        match object {
            Object::Sprite(s) => Renderer::draw_sprite(s, vp),
            Object::Text(t) => Renderer::draw_text(t, vp),
            Object::Particle(_p) => Renderer::draw_particle() 
        }
    }
    // engine state
    pub fn is_working(&self) -> bool { 
        !self.window.should_close()
    }

    pub fn stop_working(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn tick(&mut self) {
        self.time = SystemTime::now()
    }

    pub fn get_frametime(&self) -> u32 {
        self.time.elapsed().unwrap().as_nanos() as u32
    }

    pub fn get_cursor_pos(&mut self) -> Vector2<f32> {
        let pos = self.window.get_cursor_pos();
        vec2(pos.0 as f32, (pos.1 as f32 - 800.0) * -1.0)
    } 
}