use super::*;

pub enum MenuState {
    Main,
    Settings,
    Credits
}

pub struct MainMenu <'a> {
    bg: Sprite<'a>,
    main: Main<'a>,
    settings: Settings<'a>,
    credits: Credits<'a>,
    pub state: MenuState
}

impl <'a> MainMenu <'a> {
    pub fn new(shader: &'a Shader) -> MainMenu<'a> {
        let mut bg = Sprite::new("menu_bg.png", gl::NEAREST as i32, shader, false);
        bg.set_size(&vec2(2200.0, 1200.0));

        let mut b1 = Button::new("log1_play.png", "log2_play.png", shader);
        let mut b2 = Button::new("log1_set.png", "log2_set.png", shader);
        let mut b3 = Button::new("log1_cred.png", "log2_cred.png", shader);
        let mut b4 = Button::new("log1_exit.png", "log2_exit.png", shader);
        b1.set_size(&vec2(315.0, 95.0));
        b2.set_size(&vec2(315.0, 95.0));
        b3.set_size(&vec2(315.0, 95.0));
        b4.set_size(&vec2(315.0, 95.0));
        b1.set_position(&vec3(600.0 - 157.5, 600.0, -1.0));
        b2.set_position(&vec3(600.0 - 157.5, 450.0, -1.0));
        b3.set_position(&vec3(600.0 - 157.5, 300.0, -1.0));
        b4.set_position(&vec3(600.0 - 157.5, 150.0, -1.0));

        let main = Main {
            b1: b1,
            b2: b2,
            b3: b3,
            b4: b4
        };

        let mut b1 = Button::new("log1_vol-.png", "log2_vol-.png", shader);
        b1.set_size(&vec2(165.0, 95.0));
        b1.set_position(&vec3(600.0 - 157.5 - 165.0 - 30.0, 600.0, -1.0));

        let mut b2 = Button::new("log1_vol+.png", "log2_vol+.png", shader);
        b2.set_size(&vec2(155.0, 65.0));
        b2.set_position(&vec3(600.0 + 157.5 + 30.0, 600.0, -1.0));

        let mut b3 = Button::new("log1_vol.png", "log1_vol.png", shader);
        b3.set_size(&vec2(315.0, 95.0));
        b3.set_position(&vec3(600.0 - 157.5, 600.0, -1.0));

        let mut b4 = Button::new("log1_back.png", "log2_back.png", shader);
        b4.set_size(&vec2(315.0, 95.0));
        b4.set_position(&vec3(600.0 - 157.5, 450.0, -1.0));

        let settings = Settings {
            b1: b1,
            b2: b2,
            b3: b3, 
            b4: b4
        };

        let mut s = Sprite::new("credits.png", gl::NEAREST as i32, shader, false);
        s.set_size(&vec2(675.0, 3060.0));
        s.set_position(&vec3(263.0, 0.0, -1.0));

        let credits = Credits {
            s: s,
            scroll_timer: 0
        };

        MainMenu {
            bg: bg,
            main: main,
            settings: settings,
            state: MenuState::Main,
            credits: credits
        }
    }

    pub fn update(
        &mut self, 
        engine: &mut Engine, 
        events: &EventT, 
        vp: &Matrix4<f32>, 
        g_state: &mut game::GameState
    ) {
        let mut cursor = engine.get_cursor_pos();
        cursor.x -= 600.0;
        cursor.y -= 400.0;
        
        let bg_pos_x = cursor.x / 8.0 + 500.0;
        let bg_pos_y = cursor.y / 16.0 + 100.0;
        if bg_pos_x > 0.0 && bg_pos_x < 1000.0 {
            self.bg.set_position_x(-bg_pos_x);
        }
        self.bg.set_position_y(-bg_pos_y);
        
        let mut bg = Object::Sprite(&self.bg);
        engine.render_object(&mut bg, vp);
        match self.state {
            MenuState::Main => self.main.update(engine, events, vp, &mut(self.state), g_state),
            MenuState::Settings => self.settings.update(engine, events, vp, &mut(self.state)),
            MenuState::Credits => self.credits.update(engine, &mut (self.state), events, vp)
        }
    }
}

pub struct Main <'a> {
    b1: Button<'a>,
    b2: Button<'a>,
    b3: Button<'a>,
    b4: Button<'a>
}

impl <'a> Main <'a> {
    pub fn update(&mut self, engine: &mut Engine, _events: &EventT, vp: &Matrix4<f32>, m_state: &mut MenuState, g_state: &mut game::GameState) {
        let cursor = engine.get_cursor_pos();
        let cursor_vec3 = vec3(cursor.x, cursor.y, 0.0);

        let mouse_down = engine.get_mouse_press(0);

        if cursor_vec3.collides(&(self.b1)) {
            engine.render_object(&mut Object::Sprite(&(self.b1.focused)), vp);
            if mouse_down {
                *g_state = game::GameState::Game;
            }
        } else {
            engine.render_object(&mut Object::Sprite(&(self.b1.unfocused)), vp);
        }

        if cursor_vec3.collides(&(self.b2)) {
            engine.render_object(&mut Object::Sprite(&(self.b2.focused)), vp);
            if mouse_down {
                *m_state = MenuState::Settings;
            }
        } else {
            engine.render_object(&mut Object::Sprite(&(self.b2.unfocused)), vp);
        }

        if cursor_vec3.collides(&(self.b3)) {
            engine.render_object(&mut Object::Sprite(&(self.b3.focused)), vp);
            if mouse_down {
                *m_state = MenuState::Credits;
            }
        } else {
            engine.render_object(&mut Object::Sprite(&(self.b3.unfocused)), vp);
        }

        if cursor_vec3.collides(&(self.b4)) {
            engine.render_object(&mut Object::Sprite(&(self.b4.focused)), vp);
            if mouse_down {
                engine.stop_working();
            }
        } else {
            engine.render_object(&mut Object::Sprite(&(self.b4.unfocused)), vp);
        }
    }
}

pub struct Settings<'a> {
    b1: Button<'a>,
    b2: Button<'a>,
    b3: Button<'a>,
    b4: Button<'a>
}

impl <'a> Settings <'a> {
    pub fn update(&mut self, engine: &mut Engine, events: &EventT, vp: &Matrix4<f32>, m_state: &mut MenuState) {
        let cursor = engine.get_cursor_pos();
        let cursor_vec3 = vec3(cursor.x, cursor.y, 0.0);

        let mouse_down = engine.get_mouse_press(0);

        for event in events.iter() {
            if let WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                *m_state = MenuState::Main;
            }
        }

        if cursor_vec3.collides(&(self.b1)) {
            engine.render_object(&mut Object::Sprite(&(self.b1.focused)), vp);
            if mouse_down {
                engine.dec_vol();
            }
        } else {
            engine.render_object(&mut Object::Sprite(&(self.b1.unfocused)), vp);
        }

        if cursor_vec3.collides(&(self.b2)) {
            engine.render_object(&mut Object::Sprite(&(self.b2.focused)), vp);
            if mouse_down {
                engine.inc_vol();
            }
        } else {
            engine.render_object(&mut Object::Sprite(&(self.b2.unfocused)), vp);
        }

        engine.render_object(&mut Object::Sprite(&(self.b3.focused)), vp);

        if cursor_vec3.collides(&(self.b4)) {
            engine.render_object(&mut Object::Sprite(&(self.b4.focused)), vp);
            if mouse_down {
                *m_state = MenuState::Main;
            }
        } else {
            engine.render_object(&mut Object::Sprite(&(self.b4.unfocused)), vp);
        }
    }
}

struct Credits<'a> {
    s: Sprite<'a>,
    scroll_timer: i128
}

impl <'a> Credits <'a> {
    pub fn update(&mut self, engine: &mut Engine, menu_state: &mut MenuState, events: &EventT, vp: &Matrix4<f32>) {
        if self.scroll_timer < 0 {
            self.scroll_timer = 1_000_000 * 100;
            self.s.move_position_y(1.0);
        }

        for event in events {
            if let WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                *menu_state = MenuState::Main;
            }
        }

        if self.s.get_position().y > 2000.0 {
            *menu_state = MenuState::Main;
        }

        self.scroll_timer -= engine.get_frametime() as i128;
        engine.render_object(&mut Object::Sprite(&(self.s)), vp)
    }
}