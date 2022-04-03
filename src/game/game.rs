use super::*;

pub struct Game<'a> {
    main_menu: MainMenu<'a>,
    pub engine: &'a mut Engine,
    pub event_handler: &'a mut EventHandler,
    pub camera: Camera
}

impl <'a> Game <'a> {
    pub fn new(
        engine: &'a mut Engine, 
        event_handler: &'a mut EventHandler, 
        sprite_shader: &'a Shader
    ) -> Game<'a> {
        Game {
            main_menu: MainMenu::new(sprite_shader),
            engine: engine,
            event_handler: event_handler,
            camera: Camera::new()
        }
    }

    pub fn update(&mut self) {
        self.engine.pre_render();
        let vp = self.camera.get_vp();

        self.main_menu.update(self.engine, &vp);
        self.engine.tick();
    }
}