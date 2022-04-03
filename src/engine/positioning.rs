use super::*;

pub trait Positioning {
    fn get_position(&self) -> &Position;
    fn get_size(&self) -> &Size;
    fn set_position(&mut self, pos: &Position);
    fn set_size(&mut self, size: &Size);
}

impl Positioning for Position {
    fn get_position(&self) -> &Position {
        self
    }

    fn set_position(&mut self, pos: &Position) {
        *self = pos.clone();
    }

    fn get_size(&self) -> &Size {
        &NO_SIZE
    }

    fn set_size(&mut self, _: &Size) { }
} 