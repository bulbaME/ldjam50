extern crate glfw;
extern crate gl;
extern crate cgmath;

use ldjam50::engine as eng;

fn main() {
    let (mut engine, mut event_handler) = eng::init();
    
    let mut image = eng::Object::new("test.jpg", (50, 50));
    let mut image2 = eng::Object::new("test2.jpg", (160, 300));
    image2.set_position((250, 250, -1));
    
    engine.bind_object(&image);
    engine.bind_object(&image2);
    engine.set_camera((100, 200, 0));

    #[allow(unused_labels)]
    'main_loop: while engine.is_working() {
        for _event in event_handler.get() {

        }

        engine.tick();
    }
}