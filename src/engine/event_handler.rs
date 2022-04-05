use super::*;

use glfw;
use glfw::WindowEvent;
use std::sync::mpsc::Receiver;

pub fn init(events: Receiver<(f64, WindowEvent)>, glfw: Glfw) -> EventHandler {
    EventHandler {
        events, 
        glfw
    }
}

pub struct EventHandler {
    events: Receiver<(f64, WindowEvent)>,
    glfw: Glfw
}

impl EventHandler {
    fn pull(&mut self) {
        self.glfw.poll_events();
    }

    pub fn get(&mut self) -> Vec<WindowEvent> {
        self.pull();
        glfw::flush_messages(&self.events)
            .map(|v| v.1).collect()
    }
}