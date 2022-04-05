extern crate glfw;
extern crate gl;
extern crate cgmath;

use ldjam50::engine::*;
use ldjam50::game::*;

fn main() {
    let (mut engine, mut event_handler) = init();

    let sprite_shader = Shader::new("base.vert", "base.frag");
    let text_shader = Shader::new("text.vert", "text.frag");

    engine.load_sound("test.wav", "blup");

    let mut game = Game::new(
        &mut engine, 
        &mut event_handler,
        &sprite_shader, 
        &text_shader
    );

    #[allow(unused_labels)]
    'main_loop: while game.engine.is_working() {    
        game.update();

        // for event in game.event_handler.get() {
        //     match event {
        //         WindowEvent::Key(Key::Escape, _, Action::Press, _) => game.engine.stop_working(),
        //         WindowEvent::Key(Key::Space, _, Action::Press, _) => game.engine.play_sound("blup"),
        //         _ => ()
        //     }
        // }

    }
}