extern crate glfw;
extern crate gl;
extern crate cgmath;

use glfw::{Key, Action, WindowEvent};
use ldjam50::engine::*;
use ldjam50::game::*;

fn main() {
    let (mut engine, mut event_handler) = init();

    let shader = Shader::new("base.vert", "base.frag");

    let mut game = Game::new(&mut engine, &mut event_handler, &shader);

    #[allow(unused_labels)]
    'main_loop: while game.engine.is_working() {    
        game.update();

        for (_, event) in game.event_handler.get() {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => game.engine.stop_working(),
                WindowEvent::Key(Key::Space, _, Action::Press, _) => println!("{}", game.engine.get_frametime()),
                _ => ()
            }
        }

    }
}