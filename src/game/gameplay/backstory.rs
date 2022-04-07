use super::*;

pub struct Backstory <'a> {
    texts: Vec<Text<'a>>,
    text: &'static str,
    skip: Sprite<'a>,
    timer: i128,
    state: i32,
    offset: Position,
    index: usize,
    shader: &'a Shader
}

impl <'a> Backstory <'a> {
    pub fn new(sprite_shader: &'a Shader, text_shader: &'a Shader) -> Backstory<'a> {
        let text = "Once apon a time lumberjack father and lumberjack son
        \nwere leaving in the woods. They were protecting their forest from
        \nghosts and were keeping alive plants.
        \n\nBut suddenly father was taken by death. Lumberjack son was very sad,
        \nbecause his father was the dearest person to him. 
        \n\nThe year before lumberjack father and lumberjack son planted a tree 
        \nin their backyard. Now this tree was only reminder to lumberjack son
        \nabout his father.
        \n\nHelp lumberjack save his father's tree... 
        \nor at least make your best.
        ";

        let mut s = Sprite::new("skip.png", gl::NEAREST as i32, sprite_shader, false);
        s.set_size(&vec2(388.0, 76.0));
        s.set_position(&vec3(800.0, 20.0, -1.0));

        let mut text_o = Text::new("font.png", "", text_shader);
        let off = vec3(20.0, 740.0, -1.0);
        text_o.set_position(&off);

        Backstory {
            texts: vec![text_o],
            text: text,
            skip: s,
            timer: 0,
            state: 0,
            offset: off,
            index: 0,
            shader: text_shader
        }
    } 

    pub fn update(&mut self, engine: &mut Engine, events: &EventT, g_state: &mut gameplay::GameState) {
        if self.state == 2 {
            *g_state = gameplay::GameState::Gameplay;
            return;
        }

        for event in events {
            if let WindowEvent::Key(Key::Space, _, Action::Press, _) = event {
                *g_state = gameplay::GameState::Gameplay;
            }
        }

        self.next_char(false, engine);

        let vp = Camera::new().get_vp();
        for t in self.texts.iter() {
            engine.render_object(&mut Object::Text(t), &vp);
        }

        if self.state < 2 {
            engine.render_object(&mut Object::Sprite(&(self.skip)), &vp);
        }
    }

    fn next_char(&mut self, skip: bool, engine: &mut Engine) {
        self.timer -= engine.get_frametime() as i128;
        if self.timer > 0 && !skip {
            return;
        }

        self.timer = 1_000_000 * 100;
        let ch = self.text.as_bytes().get(self.index);
        if let None = ch {
            self.state = 1;
            return;
        }
        let ch = ch.unwrap();
        
        if ch == &('\n' as u8) {
            let mut new_offset = self.offset;
            new_offset.y -= 30.0;
            self.offset = new_offset;
            
            let mut text = Text::new("font.png", "", self.shader);
            text.set_position(&self.offset);
            self.texts.push(text);
        } else {
            let text_o = self.texts.last_mut().unwrap();
            let text = format!("{}{}", text_o.get_text(), *ch as char);
            text_o.set_text(&text);
        }

        self.index += 1;
    }
}