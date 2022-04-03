use super::*;

pub trait Collision {
    fn collides(&self, obj: &impl Positioning) -> bool;
}

impl<T: Positioning> Collision for T {
    fn collides(&self, obj: &impl Positioning) -> bool {
        let p1 = self.get_position();
        let p2 = obj.get_position();
        let s1 = self.get_size();
        let s2 = obj.get_size();

        p1[0] < p2[0] + s2[0] &&
        p1[0] + s1[1] > p2[0] &&
        p1[1] < p2[1] + s2[1] &&
        p1[1] + s1[1] > p2[1] 
    }
}