use super::*;
use shader::SetType;

pub struct Renderer {}

impl Renderer {
    // pub fn new() -> Renderer {
    //     // TODO: Default texture support
    //     unsafe {
    //         gl::Viewport(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);            
    //     }

    //     Renderer {
    //         shader: Shader::new("base.vert", "base.frag")
    //     }
    // }

    pub fn draw_sprite(sprite: &Sprite, vp: &Matrix4<f32>) {
        let texture = sprite.get_texture();
        let mesh = sprite.get_mesh();
        let size = sprite.get_size();
        let position = sprite.get_position();
        let shader = sprite.get_shader();

        // Bindings
        texture.bind(0);
        shader.bind();

        // Calculate the MVP
        let mut model = Matrix4::<f32>::identity();
        model = model * Matrix4::from_translation(position.clone());
        // model = model * Matrix4::<f32>::from_scale(size.x);
        model.x.x *= size.x;  // scale x
        model.y.y *= size.y;  // scale y


        let mvp: Matrix4<f32> = vp * model;

        // Set uniforms 
        shader.set("uTexture", SetType::Int(0)); 
        shader.set("uMVP", SetType::Mat4(&mvp)); 
        
        unsafe {
            // Bind the mesh
            gl::BindVertexArray(mesh.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ebo); // it is unnecessary to have it here

            // Draw the quad
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);

            // Unbind the mesh
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }

        // Unbindings
        texture.unbind();
        shader.unbind();
    }
 
    pub fn draw_text(text: &Text, vp: &Matrix4<f32>) {
        let bitmap_font = text.get_font();
        let shader = text.get_shader();
        let mesh = text.get_mesh();
        
        let bitmap_size = bitmap_font.get_size();
        let bitmap_width = bitmap_size.x;
        let bitmap_height = bitmap_size.y;

        let position = text.get_position();
        let size = text.get_size();

        // Bind bitmap texture
        bitmap_font.bind(0);

        // Bind default text shader
        shader.bind();

        // Const text params
        let start_char = ' ' as u8;
        let char_in_row = 18;

        // Size in pixel
        let char_width = 15.0;
        let char_height = 21.0;

        // Size in ratio
        let char_r_width = char_width / bitmap_width; // value <-- 0.055
        let char_r_height = char_height / bitmap_height; // value <-- 0.166

        // Current text params
        let mut advance: f32 = 0.0;

        for c in text.get_text().as_bytes().iter() {
            
            // Get current char index
            let current = c - start_char; // try checking table

            // Position
            let pos_x = ((current % char_in_row) as f32 * char_width) / bitmap_width as f32;
            let pos_y = (bitmap_height - (((current / char_in_row) as f32 * char_height) + char_height)) / bitmap_height;
            
            // Set source rect
            let x_ = pos_x + char_r_width;
            let y_ = pos_y + char_r_height;
            let source = Matrix4::<f32>::from_cols(
                vec4(pos_x, x_, x_, pos_x),
                vec4(pos_y, pos_y, y_, y_), 
                vec4(0.0, 0.0, 0.0, 0.0),
                vec4(0.0, 0.0, 0.0, 0.0)
            );

            // Draw the text
            let mut model = Matrix4::<f32>::identity();
            model = model * Matrix4::from_translation(vec3(position.x + advance, position.y, position.z));
            model.x.x *= char_width * size.x;
            model.y.y *= char_height * size.y;


            // Create mvp
            let mvp = vp * model;

            // Set uniform
            shader.set("uColor", SetType::Vec4(text.get_color()));
            shader.set("uSource", SetType::Mat4(&source));
            shader.set("uMVP", SetType::Mat4(&mvp));
            shader.set("uTexture", SetType::Int(0));

            unsafe {
                // Bind the mesh
                gl::BindVertexArray(mesh.vao);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ebo);
    
                // Draw the quad
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);
    
                // Unbind the mesh
                gl::BindVertexArray(0);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            }
            
            advance += char_width * size.x;

            let char_c: char = *c as char;
            if char_c != ' ' || char_c != '!' || char_c != '.' || char_c != ',' {
                advance += char_width * size.x * 0.16; 
            }
        }

        bitmap_font.unbind();
        shader.unbind();
    }
    
    pub fn draw_particle() {

    }
}