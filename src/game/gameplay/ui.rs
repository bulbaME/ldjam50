use super::*;

pub struct UI <'a> {
    frames: [Sprite<'a>; 4],
    tools: [Sprite<'a>; 4],
    curr_tools: [Sprite<'a>; 4],
    pub current: i32,
    camera: Camera,
    bars: [Sprite<'a>; 4],
    pub colors: [Sprite<'a>; 4]
}

impl <'a> UI <'a> {
    pub fn new(shader: &Shader) -> UI {
        let mut s1 = Sprite::new("frame.png", gl::NEAREST as i32, shader, false);
        let mut s2 = Sprite::new("frame.png", gl::NEAREST as i32, shader, false);
        let mut s3 = Sprite::new("frame.png", gl::NEAREST as i32, shader, false);
        let mut s4 = Sprite::new("frame.png", gl::NEAREST as i32, shader, false);

        s1.set_size(&vec2(128.0, 128.0));
        s2.set_size(&vec2(128.0, 128.0));
        s3.set_size(&vec2(128.0, 128.0));
        s4.set_size(&vec2(128.0, 128.0));
        s1.set_position(&vec3(1040.0, 640.0, -1.0));
        s2.set_position(&vec3(1040.0, 510.0, -1.0));
        s3.set_position(&vec3(1040.0, 380.0, -1.0));
        s4.set_position(&vec3(1040.0, 250.0, -1.0));

        let frames = [s1, s2, s3, s4];

        let mut s1 = Sprite::new("log_bar.png", gl::NEAREST as i32, shader, false);
        let mut s2 = Sprite::new("log_bar.png", gl::NEAREST as i32, shader, false);
        let mut s3 = Sprite::new("log_bar.png", gl::NEAREST as i32, shader, false);
        let mut s4 = Sprite::new("log_bar.png", gl::NEAREST as i32, shader, false);

        s1.set_size(&vec2(160.0, 47.0));
        s2.set_size(&vec2(160.0, 47.0));
        s3.set_size(&vec2(160.0, 47.0));
        s4.set_size(&vec2(160.0, 47.0));
        s1.set_position(&vec3(50.0, 730.0, -1.0));
        s2.set_position(&vec3(220.0, 730.0, -1.0));
        s3.set_position(&vec3(390.0, 730.0, -1.0));
        s4.set_position(&vec3(560.0, 730.0, -1.0));

        let bars = [s1, s2, s3, s4];

        let mut s1 = Sprite::new("red.png", gl::NEAREST as i32, shader, false);
        let mut s2 = Sprite::new("green.png", gl::NEAREST as i32, shader, false);
        let mut s3 = Sprite::new("blue.png", gl::NEAREST as i32, shader, false);
        let mut s4 = Sprite::new("orange.png", gl::NEAREST as i32, shader, false);

        s1.set_size(&vec2(120.0, 20.0));
        s2.set_size(&vec2(120.0, 20.0));
        s3.set_size(&vec2(120.0, 20.0));
        s4.set_size(&vec2(120.0, 20.0));
        s1.set_position(&vec3(70.0, 740.0, -1.0));
        s2.set_position(&vec3(240.0, 740.0, -1.0));
        s3.set_position(&vec3(410.0, 740.0, -1.0));
        s4.set_position(&vec3(580.0, 740.0, -1.0));

        let colors = [s1, s2, s3, s4];

        let mut s1 = Sprite::new("sword.png", gl::NEAREST as i32, shader, false);
        let mut s2 = Sprite::new("fireex.png", gl::NEAREST as i32, shader, false);
        let mut s3 = Sprite::new("bucket.png", gl::NEAREST as i32, shader, false);
        let mut s4 = Sprite::new("powder.png", gl::NEAREST as i32, shader, false);

        s1.set_size(&vec2(128.0, 128.0));
        s2.set_size(&vec2(128.0, 128.0));
        s3.set_size(&vec2(128.0, 128.0));
        s4.set_size(&vec2(128.0, 128.0));
        s1.set_position(&vec3(1040.0, 640.0, -1.0));
        s2.set_position(&vec3(1040.0, 510.0, -1.0));
        s3.set_position(&vec3(1040.0, 380.0, -1.0));
        s4.set_position(&vec3(1040.0, 250.0, -1.0));

        let tools = [s1, s2, s3, s4];

        let mut s1 = Sprite::new("sword.png", gl::NEAREST as i32, shader, false);
        let mut s2 = Sprite::new("fireex.png", gl::NEAREST as i32, shader, false);
        let mut s3 = Sprite::new("bucket.png", gl::NEAREST as i32, shader, false);
        let mut s4 = Sprite::new("powder.png", gl::NEAREST as i32, shader, false);

        s1.set_size(&vec2(128.0, 128.0));
        s2.set_size(&vec2(128.0, 128.0));
        s3.set_size(&vec2(128.0, 128.0));
        s4.set_size(&vec2(128.0, 128.0));

        let curr_tools = [s1, s2, s3, s4];

        UI {
            frames: frames,
            tools: tools,
            curr_tools: curr_tools,
            current: 0,
            camera: Camera::new(),
            bars: bars,
            colors: colors
        }
    }

    pub fn update(&mut self, engine: &mut Engine) {
        let vp = self.camera.get_vp();

        let cursor = engine.get_cursor_pos();
        let cursor_vec3 = vec3(cursor.x, cursor.y, -1.0);
        let mouse_down = engine.get_mouse_press(0);

        for i in 0..self.frames.len() {
            let mut object = Object::Sprite(&(self.frames[i]));
            engine.render_object(&mut object, &vp);

            if mouse_down {
                if cursor_vec3.collides(&(self.frames[i])) {
                    if self.current == i as i32 + 1 {
                        self.current = 0;
                    } else {
                        self.current = i as i32 + 1;
                    }
                }
            }
        }

        self.colors.iter().for_each(|s| engine.render_object(&mut Object::Sprite(s), &vp));
        self.bars.iter().for_each(|s| engine.render_object(&mut Object::Sprite(s), &vp));

        for t in self.tools.iter() {
            let mut object = Object::Sprite(t);
            engine.render_object(&mut object, &vp);
        }

        if self.current >= 1 {
            let s = &mut (self.curr_tools[(self.current-1) as usize]);
            let mut cursor_vec3 = cursor_vec3;
            cursor_vec3.x -= 50.0;
            cursor_vec3.y -= 50.0;

            s.set_position(&cursor_vec3);
            let mut object = Object::Sprite(s);
            engine.render_object(&mut object, &vp);
        }
    }
}