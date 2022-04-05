use super::*;

pub enum GameState {
    Menu,
    Game
}

pub struct Game<'a> {
    main_menu: MainMenu<'a>,
    game: Gameplay<'a>,
    pub engine: &'a mut Engine,
    pub event_handler: &'a mut EventHandler,
    pub camera: Camera,
    fps_meter: Text<'a>,
    state: GameState
}

impl <'a> Game <'a> {
    pub fn new(
        engine: &'a mut Engine, 
        event_handler: &'a mut EventHandler, 
        sprite_shader: &'a Shader,
        text_shader: &'a Shader
    ) -> Game<'a> {
        let mut text = Text::new("font.png", "", text_shader);
        text.set_color(&vec4(1.0, 1.0, 1.0, 0.5));
        Game {
            main_menu: MainMenu::new(sprite_shader),
            game: Gameplay::new(sprite_shader),
            engine: engine,
            event_handler: event_handler,
            camera: Camera::new(),
            fps_meter: text,
            state: GameState::Menu
        }
    }

    pub fn update(&mut self) {
        self.engine.pre_render();
        let vp = self.camera.get_vp();
        let events = self.event_handler.get();

        // fps meter
        let frame_time = self.engine.get_frametime() as f64 / 1_000_000.0; // in milliseconds
        let fps = (1000.0 / frame_time) as i32;
        let fps = fps.to_string();
        self.fps_meter.set_text(&fps);
        let fps_meter = Object::Text(&(self.fps_meter));

        match self.state {
            GameState::Menu => self.main_menu.update(self.engine, &events, &vp, &mut(self.state)),
            GameState::Game => self.game.update(self.engine, &events, &vp, &mut(self.state))
        }

        self.engine.render_object(&fps_meter, &vp);
        self.engine.tick();
    }

    pub fn start_game() {

    }
}