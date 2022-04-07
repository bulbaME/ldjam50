use super::*;

use rand;

pub enum GameState {
    Backstory,
    Gameplay,
    Lost
}

pub struct Gameplay <'a> {
    paused: bool,
    pub state: GameState,
    counter: u128,   // nanoseconds
    bg: [Sprite<'a>; 4],
    camera: Camera,
    saplings: [Sprite<'a>; 4],
    sapling_hp: f32,
    sapling_burining: bool,
    score: i32,
    energy: f32,
    water: f32,
    growth: f32,
    ui: UI<'a>,
    enemies: Vec<Enemy<'a>>,
    sprite_shader: &'a Shader,
    particle_shader: &'a Shader,
    particles_f: particle::Fire<'a>,
    particles_fex: Vec<particle::Fireex<'a>>,
    particles_pwd: Vec<particle::Powder<'a>>,
    particles_wtr: Vec<particle::Water<'a>>,
    particles_dmg: Vec<particle::Damage<'a>>,
    enemy_timer: i128,
    water_timer: i128,
    growth_timer: i128,
    energy_timer: i128,
    fire_timer: i128,
    pause_s: Sprite<'a>,
    pause_res: Button<'a>,
    pause_exit: Button<'a>,
    pause_rest: Button<'a>,
    lost_s: Sprite<'a>,
    backstory: Backstory<'a>
}

impl <'a> Gameplay <'a> {
    pub fn new(sprite_shader: &'a Shader, particle_shader: &'a Shader, text_shader: &'a Shader) -> Gameplay<'a> {
        let mut s1 = Sprite::new("game_bg1.png", gl::NEAREST as i32, sprite_shader, false);
        let mut s2 = Sprite::new("game_bg2.png", gl::NEAREST as i32, sprite_shader, false);
        let mut s3 = Sprite::new("game_bg3.png", gl::NEAREST as i32, sprite_shader, false);
        let mut s4 = Sprite::new("game_bg4.png", gl::NEAREST as i32, sprite_shader, false);
        
        s1.move_position_z(-3.0);
        s2.move_position_z(-3.0);
        s3.move_position_z(-3.0);
        s4.move_position_z(-3.0);

        let bg = [s1, s2, s3, s4];

        let mut s1 = Sprite::new("sapling_1.png", gl::NEAREST as i32, sprite_shader, false);
        let mut s2 = Sprite::new("sapling_2.png", gl::NEAREST as i32, sprite_shader, false);
        let mut s3 = Sprite::new("sapling_3.png", gl::NEAREST as i32, sprite_shader, false);
        let mut s4 = Sprite::new("sapling_4.png", gl::NEAREST as i32, sprite_shader, false);
        
        s1.set_position(&vec3(610.0, 70.0, -1.0));
        s2.set_position(&vec3(610.0, 70.0, -1.0));
        s3.set_position(&vec3(610.0, 70.0, -1.0));
        s4.set_position(&vec3(610.0, 70.0, -1.0));

        let saplings = [s1, s2, s3, s4];

        let mut fire = particle::Fire::new(particle_shader);
        fire.emmiter.spawn_time = 0.0;
        fire.set_position(&vec3(689.0, 120.0, -1.0));

        let mut s1 = Sprite::new("paused.png", gl::NEAREST as i32, sprite_shader, false);

        let mut b1 = Button::new("log1_resume.png", "log2_resume.png", sprite_shader);
        let mut b2 = Button::new("log1_restart.png", "log2_restart.png", sprite_shader);
        let mut b3 = Button::new("log1_exit.png", "log2_exit.png", sprite_shader);

        s1.set_size(&vec2(230.0, 45.0));
        b1.set_size(&vec2(315.0, 95.0));
        b2.set_size(&vec2(315.0, 95.0));
        b3.set_size(&vec2(315.0, 95.0));
        s1.set_position(&vec3(600.0 - 115.0, 600.0, -1.0));
        b1.set_position(&vec3(600.0 - 157.5, 450.0, -1.0));
        b2.set_position(&vec3(600.0 - 157.5, 300.0, -1.0));
        b3.set_position(&vec3(600.0 - 157.5, 150.0, -1.0));

        let mut lost_s = Sprite::new("lost.png", gl::NEAREST as i32, sprite_shader, false);
        lost_s.set_size(&vec2(435.0, 70.0));
        lost_s.set_position(&vec3(383.0, 365.0, -1.0));

        Gameplay {
            paused: false,
            state: GameState::Backstory,
            counter: 1,
            bg: bg,
            saplings: saplings,
            sapling_hp: 99.9,
            sapling_burining: false,
            score: 0,
            energy: 100.0,
            camera: Camera::new(),
            ui: UI::new(sprite_shader),
            enemies: vec![],
            sprite_shader: sprite_shader,
            particle_shader: particle_shader,
            growth: 100.0,
            water: 100.0,
            particles_f: fire,
            particles_fex: vec![],
            particles_pwd: vec![],
            particles_wtr: vec![],
            particles_dmg: vec![],
            enemy_timer: 0,
            energy_timer: 0,
            water_timer: 0,
            fire_timer: 0,
            growth_timer: 0,
            pause_s: s1,
            pause_res: b1,
            pause_rest: b2,
            pause_exit: b3,
            lost_s: lost_s,
            backstory: Backstory::new(sprite_shader, text_shader)
        }
    }

    pub fn restart(&mut self) {
        self.counter = 1;
        self.sapling_hp = 99.9;
        self.sapling_burining = false;
        self.score = 0;
        self.energy = 100.0;
        self.growth = 100.0; 
        self.water = 100.0;
        self.particles_f.emmiter.spawn_time = 0.0;
        self.paused = false;
    }

    pub fn update(
        &mut self, 
        engine: &mut Engine, 
        events: &EventT, 
        g_state: &mut game::GameState,
        m_state: &mut main_menu::MenuState,
        cr: &mut bool
    ) {
        for event in events {
            if let WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                self.paused = !self.paused;
            }
        }

        let cursor = engine.get_cursor_pos();
        let vp = self.camera.get_vp();

        // background paralax
        {
            let mut cursor = cursor;
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
        }

        for bg in self.bg.iter().rev() {
            let mut object = Object::Sprite(bg);
            engine.render_object(&mut object, &vp);
        }

        let i = 3 - self.sapling_hp as i32 / 25;
        let mut sapling = Object::Sprite(&(self.saplings[i as usize]));
        engine.render_object(&mut sapling, &vp);

        match self.state {
            GameState::Gameplay => self.update_game(engine, events, g_state),
            GameState::Backstory => self.backstory.update(engine, events, &mut self.state),
            GameState::Lost => {
                for event in events {
                    if let WindowEvent::Key(_, _, Action::Press, _) = event {
                        self.restart();
                        self.state = GameState::Gameplay;
                        *cr = true;
                        *m_state = main_menu::MenuState::Credits;
                        *g_state = game::GameState::Menu;
                    }
                }

                let vp = Camera::new().get_vp();
                engine.render_object(&mut Object::Sprite(&(self.lost_s)), &vp);
            }
        }
    }

    fn update_game(
        &mut self, 
        engine: &mut Engine, 
        _events: &EventT, 
        g_state: &mut game::GameState
    ) {

        let cursor = engine.get_cursor_pos();
        let cursor_vec3 = vec3(cursor.x, cursor.y, -1.0);
        let mouse_down = engine.get_mouse_press(0);
        let vp = self.camera.get_vp();

        let i = 3 - self.sapling_hp as i32 / 25;
        let mut sapling = Object::Sprite(&(self.saplings[i as usize]));
        engine.render_object(&mut sapling, &vp);

        if self.paused {
            let vp = Camera::new().get_vp();

            if cursor_vec3.collides(&(self.pause_res)) {
                engine.render_object(&mut Object::Sprite(&(self.pause_res.focused)), &vp);
                if mouse_down {
                    self.paused = false;
                }
            } else {
                engine.render_object(&mut Object::Sprite(&(self.pause_res.unfocused)), &vp);
            }

            if cursor_vec3.collides(&(self.pause_rest)) {
                engine.render_object(&mut Object::Sprite(&(self.pause_rest.focused)), &vp);
                if mouse_down {
                    self.restart();
                }
            } else {
                engine.render_object(&mut Object::Sprite(&(self.pause_rest.unfocused)), &vp);
            }

            if cursor_vec3.collides(&(self.pause_exit)) {
                engine.render_object(&mut Object::Sprite(&(self.pause_exit.focused)), &vp);
                if mouse_down {
                    *g_state = game::GameState::Menu;
                }
            } else {
                engine.render_object(&mut Object::Sprite(&(self.pause_exit.unfocused)), &vp);
            }

            engine.render_object(&mut Object::Sprite(&(self.pause_s)), &vp);

            return;
        }

        // update bars
        {
            self.ui.colors[0].set_size(&vec2(self.sapling_hp * 1.2, 20.0));
            self.ui.colors[1].set_size(&vec2(self.growth * 1.2, 20.0));
            self.ui.colors[2].set_size(&vec2(self.water * 1.2, 20.0));
            self.ui.colors[3].set_size(&vec2(self.energy * 1.2, 20.0));
        }

        // particle spawn
        if mouse_down && cursor.x < 1000.0 {
            let over_sapling = cursor_vec3.collides(&(self.saplings[0]));
            match self.ui.current {
                2 => {
                    if self.energy >= 50.0 {
                        self.energy -= 50.0;
                        engine.play_sound("fireex");
                        let mut particle = particle::Fireex::new(self.particle_shader);
                        particle.set_position(&cursor_vec3);
                        self.particles_fex.push(particle);
                        if over_sapling {
                            self.sapling_burining = false;
                            self.particles_f.emmiter.spawn_time = 0.0;
                        }
                    }
                },

                3 => {
                    if self.energy >= 20.0 {
                        if over_sapling {
                            self.water += 20.0;
                        }
    
                        self.energy -= 20.0;
                        if self.water > 100.0 {
                            self.water = 100.0;
                        }

                        engine.play_sound("water");
                        let mut particle = particle::Water::new(self.particle_shader);
                        particle.set_position(&cursor_vec3);
                        self.particles_wtr.push(particle);
                    }
                },

                4 => {
                    if self.energy >= 40.0 {
                        self.energy -= 40.0;
                        if over_sapling {
                            self.growth += 40.0;
                        }

                        if self.growth > 100.0 {
                            self.growth = 100.0;
                        }

                        engine.play_sound("powder");
                        let mut particle = particle::Powder::new(self.particle_shader);
                        particle.set_position(&cursor_vec3);
                        self.particles_pwd.push(particle);
                    }
                },

                _ => ()
            }
        }

        if self.sapling_hp < 0.0 {
            self.state = GameState::Lost;
        }        

        // spawn enemy
        if self.enemy_timer < 0 {
            let val: f32 = rand::random();
            self.enemy_timer = 1_000_000 * (10000.0 + 5000.0 * val) as i128;
            let mirrored = rand::random();
            let mut dir = -1.0;
            if mirrored {
                dir *= -1.0;
            }

            let mut enemy = Enemy::new(self.sprite_shader, mirrored, dir);

            if mirrored {
                enemy.set_position_x(-600.0);
            } else {
                enemy.set_position_x(1800.0);
            }

            self.enemies.push(enemy);
        }

        let sapling = &(self.saplings[0]);
        for e in self.enemies.iter_mut() {
            if e.collides(sapling) {
                if e.get_fired() {
                    self.particles_f.emmiter.alive = true;
                    self.particles_f.emmiter.spawn_time = 1_000_000.0;
                    self.sapling_burining = true;
                } else {
                    e.fire();
                }
            }

            if mouse_down && self.ui.current == 1 && cursor_vec3.collides(e) && self.energy >= 10.0 {
                self.energy -= 10.0;
                e.damage();
                let mut particle = particle::Damage::new(self.particle_shader);
                particle.set_position(&cursor_vec3);
                self.particles_dmg.push(particle);
                engine.play_sound("hit");
            }
        }

        // render enemies
        for i in (0..self.enemies.len()).rev() {
            self.enemies[i].update(engine, &vp);
            if self.enemies[i].is_dead() {
                self.enemies.remove(i);
            }
        }

        // update particles
        {
            self.particles_f.update(engine, &vp);

            for i in (0..self.particles_fex.len()).rev() {
                self.particles_fex[i].update(engine, &vp);
                if !self.particles_fex[i].emmiter.alive {
                    self.particles_fex.remove(i);
                }
            }

            for i in (0..self.particles_wtr.len()).rev() {
                self.particles_wtr[i].update(engine, &vp);
                if !self.particles_wtr[i].emmiter.alive {
                    self.particles_wtr.remove(i);
                }
            }

            for i in (0..self.particles_pwd.len()).rev() {
                self.particles_pwd[i].update(engine, &vp);
                if !self.particles_pwd[i].emmiter.alive {
                    self.particles_pwd.remove(i);
                }
            }

            for i in (0..self.particles_dmg.len()).rev() {
                self.particles_dmg[i].update(engine, &vp);
                if !self.particles_dmg[i].emmiter.alive {
                    self.particles_dmg.remove(i);
                }
            }
        }

        if self.fire_timer < 0 {
            self.fire_timer = 1_000_000 * 100;
            self.sapling_hp -= 1.0;
        }

        if self.growth_timer < 0 {
            self.growth_timer = 1_000_000 * 400;
            if self.growth < 0.0 {
                self.sapling_hp -= 2.0;
            } else {
                self.growth -= 1.0;
            }
        }

        if self.water_timer < 0 {
            self.water_timer = 1_000_000 * 200;
            if self.water < 0.0 {
                self.sapling_hp -= 0.5;
            } else {
                self.water -= 1.0;
            }
        }

        if self.energy_timer < 0 && self.energy < 100.0 {
            self.energy_timer = 1_000_000 * 300;
            if self.growth > 50.0 {
                self.energy += 3.0;
            } else {
                self.energy += 1.5;
            }
            if self.energy < 15.0 {
                self.energy += 5.0;
            }
        }

        self.ui.update(engine);

        let fm = engine.get_frametime() as u128;
        self.counter += fm;
        self.enemy_timer -= fm as i128;
        self.energy_timer -= fm as i128;        
        self.growth_timer -= fm as i128;        
        self.water_timer -= fm as i128;
        if self.sapling_burining {
            self.fire_timer -= fm as i128;
        }
    }
}