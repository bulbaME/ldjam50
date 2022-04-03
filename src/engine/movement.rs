use super::*;

pub trait Movement {
    fn move_position(&mut self, pos: &Position);
    fn move_position_x(&mut self, pos: f32);
    fn move_position_y(&mut self, pos: f32);
    fn move_position_z(&mut self, pos: f32);
    fn set_position_x(&mut self, pos: f32);
    fn set_position_y(&mut self, pos: f32);
    fn set_position_z(&mut self, pos: f32);
}

impl <T: Positioning> Movement for T {
    fn move_position(&mut self, pos: &Position) {
        self.move_position_x(pos[0]);
        self.move_position_y(pos[1]);
        self.move_position_z(pos[2]);
    }

    fn move_position_x(&mut self, pos: f32) {
        let prev_pos = self.get_position()[0];
        self.set_position_x(prev_pos + pos);
    }
    
    fn move_position_y(&mut self, pos: f32) {
        let prev_pos = self.get_position()[1];
        self.set_position_x(prev_pos + pos);
    }

    fn move_position_z(&mut self, pos: f32) {
        let prev_pos = self.get_position()[2];
        self.set_position_x(prev_pos + pos);
    }

    fn set_position_x(&mut self, pos: f32) {
        let mut pos_ = self.get_position().clone();
        pos_[0] = pos;
        self.set_position(&pos_);
    }

    fn set_position_y(&mut self, pos: f32) {
        let mut pos_ = self.get_position().clone();
        pos_[1] = pos;
        self.set_position(&pos_);
    }

    fn set_position_z(&mut self, pos: f32) {
        let mut pos_ = self.get_position().clone();
        pos_[2] = pos;
        self.set_position(&pos_);
    }
}