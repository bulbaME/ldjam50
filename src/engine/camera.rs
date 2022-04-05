use super::*;

pub struct Camera {
    projection: Matrix4<f32>,
    position: Position
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            projection: cgmath::ortho(0.0, WINDOW_WIDTH as f32, 0.0, WINDOW_HEIGHT as f32, 0.0, 100.0),
            position: vec3(0.0, 0.0, 0.0)
        }
    }

    pub fn get_vp(&self) -> Matrix4<f32> {
        self.projection * Matrix4::<f32>::from_translation(self.position.clone())
    }

    pub fn get_p(&self) -> &Matrix4<f32> {
        &(self.projection)
    }
}

impl Positioning for Camera {
    fn get_position(&self) -> &Position {
        &(self.position)
    }

    fn get_size(&self) -> &Size {
        &NO_SIZE
    }

    fn set_position(&mut self, pos: &Position) {
        self.position = pos.clone();
    }

    fn set_size(&mut self, _: &Size) { }
}