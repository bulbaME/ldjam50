use super::*;

static FRAME_TIME: u64 = 100; // milliseconds

enum Animation {
    Move,
    Fire,
    Die,
    Stop
}

pub struct Enemy <'a> {
    anim_move: [Sprite<'a>; 1],
    anim_fire: [Sprite<'a>; 11],
    anim_die: [Sprite<'a>; 10],
    anim_stop: [Sprite<'a>; 6],
    counter: i32,
    anim: Animation,
    hp: i32,
    anim_timer: u64,
    move_timer: u64,
    speed: f32,
    size: Size,
    position: Position,
    dead: bool
}

impl <'a> Enemy <'a> {
    pub fn new(shader: &'a Shader) -> Enemy {
        let anim_move = [
            Sprite::new("em_6.png", gl::NEAREST as i32, shader),
            // Sprite::new("em_2.png", gl::NEAREST as i32, shader),
        ];

        let anim_fire = [
            Sprite::new("ef_1.png", gl::NEAREST as i32, shader),
            Sprite::new("ef_2.png", gl::NEAREST as i32, shader),
            Sprite::new("ef_3.png", gl::NEAREST as i32, shader),
            Sprite::new("ef_4.png", gl::NEAREST as i32, shader),
            Sprite::new("ef_5.png", gl::NEAREST as i32, shader),
            Sprite::new("ef_6.png", gl::NEAREST as i32, shader),
            Sprite::new("ef_7.png", gl::NEAREST as i32, shader),
            Sprite::new("ef_8.png", gl::NEAREST as i32, shader),
            Sprite::new("ef_9.png", gl::NEAREST as i32, shader),
            Sprite::new("ef_10.png", gl::NEAREST as i32, shader),
            Sprite::new("ef_11.png", gl::NEAREST as i32, shader)
        ];

        let anim_die = [
            Sprite::new("ed_1.png", gl::NEAREST as i32, shader),
            Sprite::new("ed_2.png", gl::NEAREST as i32, shader),
            Sprite::new("ed_3.png", gl::NEAREST as i32, shader),
            Sprite::new("ed_4.png", gl::NEAREST as i32, shader),
            Sprite::new("ed_5.png", gl::NEAREST as i32, shader),
            Sprite::new("ed_6.png", gl::NEAREST as i32, shader),
            Sprite::new("ed_7.png", gl::NEAREST as i32, shader),
            Sprite::new("ed_8.png", gl::NEAREST as i32, shader),
            Sprite::new("ed_9.png", gl::NEAREST as i32, shader),
            Sprite::new("ed_10.png", gl::NEAREST as i32, shader)
        ];

        let anim_stop = [
            Sprite::new("em_6.png", gl::NEAREST as i32, shader),
            Sprite::new("em_5.png", gl::NEAREST as i32, shader),
            Sprite::new("em_4.png", gl::NEAREST as i32, shader),
            Sprite::new("em_3.png", gl::NEAREST as i32, shader),
            Sprite::new("em_2.png", gl::NEAREST as i32, shader),
            Sprite::new("em_1.png", gl::NEAREST as i32, shader)
        ];

        let mut enemy = Enemy {
            anim_move: anim_move,
            anim_fire: anim_fire,
            anim_die: anim_die,
            anim_stop: anim_stop,
            counter: 0,
            anim: Animation::Move,
            hp: 3,
            anim_timer: 0,
            move_timer: 0,
            speed: 50.0,
            size: vec2(80.0, 80.0),
            position: vec3(0.0, 0.0, -1.0),
            dead: false
        };

        enemy.set_size(&vec2(320.0, 320.0));

        enemy
    }

    pub fn damage(&mut self) {
        self.hp -= 1;
    }

    fn set_animation(&mut self, anim: Animation) {
        self.counter = 0;
        self.anim = anim;
    }

    pub fn update(
        &mut self,
        engine: &mut Engine,
        vp: &Matrix4<f32>,
    ) {
        if self.dead {
            return;
        }
        let cursor = engine.get_cursor_pos();
        let cursor_vec3 = vec3(cursor.x, cursor.y, 0.0);
        if engine.get_mouse_press(0) && cursor_vec3.collides(self) {
            self.hp -= 1;
        }


        let frame_time = engine.get_frametime();
        self.anim_timer += frame_time as u64;
        self.move_timer += frame_time as u64;

        // animation
        if self.anim_timer / 1_000_000 > FRAME_TIME {
            self.anim_timer = 0;
            match self.anim {
                Animation::Move => {
                    self.counter += 1;
                    if self.counter == self.anim_move.len() as i32 {
                        self.counter = 0;
                    }
                },

                Animation::Fire => {
                    self.counter += 1;
                    if self.counter == self.anim_fire.len() as i32 {
                        self.counter = 0;
                    }
                },

                Animation::Die => {
                    self.counter += 1;
                    if self.counter == self.anim_die.len() as i32 {
                        self.dead = true;
                        return;
                    }
                },

                Animation::Stop => {
                    self.counter += 1;
                    if self.counter == self.anim_stop.len() as i32 {
                        self.set_animation(Animation::Die);
                    }  
                }
            }
        }

        let mut object: Object;
        match self.anim {
            Animation::Move => object = Object::Sprite(&(self.anim_move[self.counter as usize])),
            Animation::Fire => object = Object::Sprite(&(self.anim_fire[self.counter as usize])),
            Animation::Die => object = Object::Sprite(&(self.anim_die[self.counter as usize])),
            Animation::Stop => object = Object::Sprite(&(self.anim_stop[self.counter as usize])),
        }

        engine.render_object(&mut object, vp);

        // movement
        if let Animation::Move = self.anim {
            if (self.move_timer / 1_000_000) as f32 / 1000.0 > 1.0 / self.speed {
                self.move_position_x(-1.0);
                self.move_timer = 0;
            }
        }

        if self.hp <= 0 {
            match self.anim {
                Animation::Die => (),
                Animation::Stop => (),
                _ => self.set_animation(Animation::Stop)
            }
        }
    }
}

impl <'a> Positioning for Enemy<'a> {
    fn get_size(&self) -> &Size {
        &(self.size)
    }
    
    fn get_position(&self) -> &Position {
        &(self.position)
    }

    fn set_size(&mut self, size: &Size) {
        self.anim_die.iter_mut().for_each(|s| (*s).set_size(size));
        self.anim_fire.iter_mut().for_each(|s| (*s).set_size(size));
        self.anim_move.iter_mut().for_each(|s| (*s).set_size(size));
        self.anim_stop.iter_mut().for_each(|s| (*s).set_size(size));
        self.size = size.clone();
    }

    fn set_position(&mut self, pos: &Position) {
        self.anim_die.iter_mut().for_each(|s| (*s).set_position(pos));
        self.anim_fire.iter_mut().for_each(|s| (*s).set_position(pos));
        self.anim_move.iter_mut().for_each(|s| (*s).set_position(pos));
        self.anim_stop.iter_mut().for_each(|s| (*s).set_position(pos));
        self.position = pos.clone();
    }
}