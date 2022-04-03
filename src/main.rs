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

    let mut bg = Sprite::new("bg2.png", gl::NEAREST as i32, &shader);
    bg.set_size(&vec2(2200.0, 1200.0));
    let mut bg = Object::Sprite(bg);

    let mut log1_unfocused = Sprite::new("log1_play.png", gl::NEAREST as i32, &shader);
    log1_unfocused.set_size(&vec2(315.0, 95.0));
    log1_unfocused.set_position(&vec3(600.0 - 157.5, 550.0, -1.0));
    let mut log1_unfocused = Object::Sprite(log1_unfocused);

    let mut log1_focused = Sprite::new("log2_play.png", gl::NEAREST as i32, &shader);
    log1_focused.set_size(&vec2(315.0, 95.0));
    log1_focused.set_position(&vec3(600.0 - 157.5, 550.0, -1.0));
    let mut log1_focused = Object::Sprite(log1_focused);

    let mut log2_unfocused = Sprite::new("log1_set.png", gl::NEAREST as i32, &shader);
    log2_unfocused.set_size(&vec2(315.0, 95.0));
    log2_unfocused.set_position(&vec3(600.0 - 157.5, 400.0, -1.0));
    let mut log2_unfocused = Object::Sprite(log2_unfocused);

    let mut log2_focused = Sprite::new("log2_set.png", gl::NEAREST as i32, &shader);
    log2_focused.set_size(&vec2(315.0, 95.0));
    log2_focused.set_position(&vec3(600.0 - 157.5, 400.0, -1.0));
    let mut log2_focused = Object::Sprite(log2_focused);

    let mut log3_unfocused = Sprite::new("log1_exit.png", gl::NEAREST as i32, &shader);
    log3_unfocused.set_size(&vec2(315.0, 95.0));
    log3_unfocused.set_position(&vec3(600.0 - 157.5, 250.0, -1.0));
    let mut log3_unfocused = Object::Sprite(log3_unfocused);

    let mut log3_focused = Sprite::new("log2_exit.png", gl::NEAREST as i32, &shader);
    log3_focused.set_size(&vec2(315.0, 95.0));
    log3_focused.set_position(&vec3(600.0 - 157.5, 250.0, -1.0));
    let mut log3_focused = Object::Sprite(log3_focused);


    #[allow(unused_labels)]
    'main_loop: while engine.is_working() {    
        let vp = camera.get_vp();
        let mut cursor = engine.get_cursor_pos();
        let cursor_vec3 = vec3(cursor.x, cursor.y, 0.0);

        engine.pre_render();
        engine.render_object(&bg, &vp);

        if let Object::Sprite(o) = &log1_unfocused {
            if cursor_vec3.collides(o) {
                engine.render_object(&log1_focused, &vp);
            } else {
                engine.render_object(&log1_unfocused, &vp);
            }
        }

        if let Object::Sprite(o) = &log2_unfocused {
            if cursor_vec3.collides(o) {
                engine.render_object(&log2_focused, &vp);
            } else {
                engine.render_object(&log2_unfocused, &vp);
            }
        }

        if let Object::Sprite(o) = &log3_unfocused {
            if cursor_vec3.collides(o) {
                engine.render_object(&log3_focused, &vp);
            } else {
                engine.render_object(&log3_unfocused, &vp);
            }
        }
        
        engine.tick();

        cursor.x -= 600.0;
        cursor.y -= 400.0;

        let bg_pos_x = cursor.x / 8.0 + 500.0;
        let bg_pos_y = cursor.y / 16.0 + 100.0;
        if let Object::Sprite(o) = &mut bg {
            if bg_pos_x > 0.0 && bg_pos_x < 1000.0 {
                o.set_position_x(-bg_pos_x);
            }
            o.set_position_y(-bg_pos_y);
        }

        for (_, event) in event_handler.get() {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => engine.stop_working(),
                WindowEvent::Key(Key::Space, _, Action::Press, _) => println!("{}", engine.get_frametime()),
                _ => ()
            }
        }

    }
}