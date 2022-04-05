extern crate rodio;

use super::*;
use std::time::SystemTime;
use std::collections::HashMap;

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};
use rodio::cpal::traits::HostTrait;

use std::io::Read;

pub fn init(window: Window) -> Engine {
    Engine {
        window: window,
        time: SystemTime::now(),
        frame_time: 0,
        sounds: HashMap::new(),
        sound_streams: vec![],
        mouse_buttons: [false; 8]
    }
}

pub struct Engine {
    pub window: Window,
    time: SystemTime,
    frame_time: u32,
    sounds: HashMap<&'static str, String>,
    sound_streams: Vec<rodio::OutputStream>,
    mouse_buttons: [bool; 8]
}

impl Engine {
    pub fn pre_render(&mut self) {
        self.window.swap_buffers();

        unsafe {
            gl::ClearColor(0.12, 0.1, 0.24, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn render_object(&mut self, object: &mut Object, vp: &Matrix4<f32>) {
        match object {
            Object::Sprite(s) => Renderer::draw_sprite(s, vp),
            Object::Text(t) => Renderer::draw_text(t, vp),
            Object::Particle(p) => p.render(self, vp)
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
        for i in 0..8 {
            let button = glfw::MouseButton::from_i32(i).unwrap();
            self.mouse_buttons[i as usize] = if let Action::Press = self.window.get_mouse_button(button) {
                true
            } else {
                false
            };
        }

        self.frame_time = self.time.elapsed().unwrap().as_nanos() as u32;
        self.time = SystemTime::now();
    }

    pub fn get_frametime(&self) -> u32 {
        self.frame_time
    }

    pub fn get_cursor_pos(&mut self) -> Vector2<f32> {
        let pos = self.window.get_cursor_pos();
        vec2(pos.0 as f32, (pos.1 as f32 - 800.0) * -1.0)
    } 

    pub fn get_mouse_press(&self, i: i32) -> bool {
        let button = glfw::MouseButton::from_i32(i).unwrap();
        self.mouse_buttons[i as usize] && if let Action::Release = self.window.get_mouse_button(button) {
            true
        } else {
            false
        }
    }

    pub fn load_sound(&mut self, path: &'static str, name: &'static str) {
        self.sounds.insert(name, ["./data/sounds/", path].concat());
    }

    pub fn play_sound(&mut self, name: &'static str) {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        self.sound_streams.push(stream);
        let path = self.sounds.get(name).unwrap();
        let file = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();
        if let Err(e) = stream_handle.play_raw(source.convert_samples()) {
            println!("Sound error: {:?}", e);
        }
    }
}