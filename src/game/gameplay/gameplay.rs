use super::*;

pub enum GameState {
    Backstory,
    Gameplay
}

pub struct Gameplay <'a> {
    paused: bool,
    pub state: GameState,
    counter: u128,   // nanoseconds
    bg: [Sprite<'a>; 4],
    camera: Camera,
    e: Enemy<'a>
}

impl <'a> Gameplay <'a> {
    pub fn new(sprite_shader: &'a Shader) -> Gameplay<'a> {
        let mut s1 = Sprite::new("game_bg1.png", gl::NEAREST as i32, sprite_shader);
        let mut s2 = Sprite::new("game_bg2.png", gl::NEAREST as i32, sprite_shader);
        let mut s3 = Sprite::new("game_bg3.png", gl::NEAREST as i32, sprite_shader);
        let mut s4 = Sprite::new("game_bg4.png", gl::NEAREST as i32, sprite_shader);
        
        s1.move_position_z(-3.0);
        s2.move_position_z(-3.0);
        s3.move_position_z(-3.0);
        s4.move_position_z(-3.0);

        let mut e = Enemy::new(sprite_shader);
        e.set_size(&vec2(200.0, 200.0));
        e.set_position(&vec3(1500.0, 200.0, -1.0));

        Gameplay {
            paused: false,
            state: GameState::Backstory,
            counter: 0,
            bg: [s1, s2, s3, s4],
            camera: Camera::new(),
            e: e
        }
    }

    pub fn update(
        &mut self, 
        engine: &mut Engine, 
        events: &EventT, 
        vp: &Matrix4<f32>, 
        g_state: &mut game::GameState
    ) {
        // background paralax
        for event in events {
            if let WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                *g_state = game::GameState::Menu;
            }
        }

        let mut cursor = engine.get_cursor_pos();
        cursor.x -= 600.0;

        let mut bg_pos_x = cursor.x / 10.0;
        
        self.camera.set_position_x(-bg_pos_x);
        if true {
            self.bg[0].set_position_x(-self.bg[0].get_size().x / 2.0 + 600.0);
        }

        bg_pos_x /= 3.0;
        if true {
            self.bg[1].set_position_x(-bg_pos_x - self.bg[0].get_size().x / 2.0 + 450.0);
        }

        bg_pos_x *= 1.5;
        if true {
            self.bg[2].set_position_x(-bg_pos_x - self.bg[0].get_size().x / 2.0 + 450.0);
        }

        bg_pos_x *= 1.5;
        if true {
            self.bg[3].set_position_x(-bg_pos_x - self.bg[0].get_size().x / 2.0 + 450.0);
        }

        let vp = self.camera.get_vp();

        for bg in self.bg.iter().rev() {
            let object = Object::Sprite(bg);
            engine.render_object(&object, &vp);
        }

        self.e.update(engine, &vp);

        self.counter += engine.get_frametime() as u128;
        // println!("{:.0}", self.counter as f64 / 1_000_000_000.0);
    }
}