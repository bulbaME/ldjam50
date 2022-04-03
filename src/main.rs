extern crate glfw;
extern crate gl;
extern crate cgmath;

use cgmath::prelude::*;
use cgmath::{vec3, vec2};

use glfw::{Key, Context, Action, WindowEvent};
use ldjam50::engine::*;

fn main() {
    let (mut engine, mut event_handler) = init();

    let shader = Shader::new("base.vert", "base.frag");
    let camera = Camera::new();
    let mut bg = Sprite::new("bg.png", gl::NEAREST as i32, &shader);

    let mut button = Sprite::new("button.png", gl::NEAREST as i32, &shader);
    button.set_position(&vec3(500.0, 500.0, -1.0));
    button.set_size(&vec2(200.0, 200.0));

    let text_shader = Shader::new("text.vert", "text.frag");
    let mut text = Text::new("font.png", "Hello! avagg agkakgj", &text_shader);
    text.set_position(&vec3(200.0, 200.0, -1.0));
    let text = Object::Text(text);

    bg.set_size(&vec2(1200.0, 800.0));
    bg.move_position_x(-200.0);
    let bg = Object::Sprite(bg);
    let button = Object::Sprite(button);

    #[allow(unused_labels)]
    'main_loop: while engine.is_working() {    
        engine.pre_render();
        engine.render_object(&bg, &camera.get_vp());
        engine.tick();

        let vp = camera.get_vp();

        let pos = engine.get_cursor_pos();
        let pos = vec3(pos[0], pos[1], 0.0);
        if let Object::Sprite(b) = &button {
            if !pos.collides(b) {
                engine.render_object(&button, &vp);
            }
        }

        engine.render_object(&text, &vp);

        for (_, event) in event_handler.get() {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => engine.stop_working(),
                WindowEvent::Key(Key::Space, _, Action::Press, _) => println!("{}", engine.get_frametime()),
                _ => ()
            }
        }

    }
}