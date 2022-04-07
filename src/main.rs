extern crate glfw;
extern crate gl;
extern crate cgmath;

use ldjam50::engine::*;
use ldjam50::game::*;

use glfw::{Context};
use cgmath::{vec2, vec3};

fn main() {
    let (mut engine, mut event_handler) = init();

    let sprite_shader = Shader::new("base.vert", "base.frag");
    let text_shader = Shader::new("text.vert", "text.frag");
    let particle_shader = Shader::new("particle.vert", "particle.frag");

    let mut loading = Sprite::new("loading.png", gl::NEAREST as i32, &sprite_shader, false);
    loading.set_size(&vec2(780.0, 156.0));
    loading.set_position(&vec3(210.0, 323.0, -1.0));
    let mut loading_obj = Object::Sprite(&loading);
    let camera = Camera::new();

    engine.pre_render();
    engine.render_object(&mut loading_obj, &(camera.get_vp()));
    engine.window.swap_buffers();

    let mut game = Game::new(
        &mut engine, 
        &mut event_handler,
        &sprite_shader, 
        &text_shader,
        &particle_shader
    );

    #[allow(unused_labels)]
    'main_loop: while game.engine.is_working() {    
        game.update();
    }
}