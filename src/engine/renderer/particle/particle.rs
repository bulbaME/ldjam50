use super::*;

pub struct Particle {
    pub position: Vec3,
    pub alive: bool,
    pub start: bool,

    pub size: Vec2,
    pub final_size: Vec2,
    pub size_change: f32,

    pub age: f32,
    pub lifespan: f32,

    pub velocity: Vec2,
    pub final_velocity: Vec2,
    pub velocity_change: Vec2,

    pub direction: Vec2,
    pub final_direction: Vec2,
    pub direction_change: Vec2,

    pub color: Vec4,
    pub final_color: Vec4,
    pub color_change: Vec4,
    pub only_affect_alpha: bool,

    pub mass: f32,
    pub final_mass: f32,
    pub mass_change: f32,
}

impl Particle {
    pub fn update(&mut self, delta: f32) {
        self.age += delta;
        if self.age >= self.lifespan {
            self.alive = false;
        }

        if !self.start {
            self.start = true;

            self.size_change = (self.final_size.x - self.size.x) / self.lifespan;
            self.velocity_change = (self.final_velocity - self.velocity) / self.lifespan;
            self.direction_change = (self.final_direction - self.direction) / self.lifespan;
            if self.only_affect_alpha {
                let mut clone = self.color.clone();
                clone.w = self.final_color.w;
                self.color_change = (clone - self.color) / self.lifespan;
            } else {
                self.color_change = (self.final_color - self.color) / self.lifespan;
            }
            
            self.mass_change = (self.final_mass - self.mass) / self.lifespan;
        }

        self.size.x += self.size_change * delta;
        self.size.y += self.size_change * delta;

        self.velocity += self.velocity_change * delta;

        self.direction += self.direction_change * delta;

        self.color += self.color_change * delta;

        self.mass += self.mass_change * delta;

        let norm = self.direction.normalize();
        self.position.x += norm.x * self.velocity.x;
        self.position.y += norm.y * (self.velocity.y - (self.mass * 0.98));
    }

    pub fn render(&self, mesh: &Mesh, shader: &Shader, vp: &Matrix4<f32>) {
        render::render(mesh, shader, self, vp);
    }
}