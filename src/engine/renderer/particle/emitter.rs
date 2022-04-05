use super::*;

pub struct Emitter {
    // Sel
    pub position: Vec3,
    pub radius: f32,

    pub age: f32,
    pub lifespan: f32,

    pub alive: bool,
    pub start: bool,

    pub particles: Vec<particle::Particle>,

    // Particles
    pub particle_size: Range<f32>,
    pub particle_final_size: Range<f32>,

    pub particle_lifespan: Range<f32>,

    pub initial_count: Range<u32>,
    pub spawn_count: Range<u32>,

    pub spawn_interval: Range<f32>,
    pub current_interval: f32,
    pub next_interval: f32,

    pub start_color: Range<Vec4>,
    pub final_color: Range<Vec4>,
    pub only_affect_alpha: bool,

    pub start_velocity: Range<Vec2>,
    pub final_velocity: Range<Vec2>,

    pub start_direction: Range<Vec2>,
    pub final_direction: Range<Vec2>,

    pub start_mass: Range<f32>,
    pub final_mass: Range<f32>,
}

impl Emitter {
    pub fn update(&mut self, mesh: &Mesh, shader: &Shader,  rnd: &mut rand::rngs::ThreadRng, delta: f32, vp: &Matrix4<f32>) {
        if !self.alive { return; }
        self.age += delta;
        if self.age >= self.lifespan {
            self.alive = false;
            return;
        }
        
        if !self.start {
            self.initial_spawn(rnd);
            self.next_interval = rnd.gen_range(self.spawn_interval.start..self.spawn_interval.end);

            self.start = true;
        }

        self.current_interval += delta;
        if self.current_interval >= self.next_interval {
            for _ in 0..rnd.gen_range(self.spawn_count.start..self.spawn_count.end) {
                self.spawn_particle(rnd);
            }

            self.next_interval = rnd.gen_range(self.spawn_interval.start..self.spawn_interval.end);
            self.current_interval = 0.0;
        }

        for i in (0..self.particles.len()).rev() {
            self.particles[i].update(delta);
            self.particles[i].render(&mesh, &shader, vp);

            if !self.particles[i].alive {
                self.particles.remove(i);
            }
        }
    }

    pub fn spawn_particle(&mut self, rnd: &mut rand::rngs::ThreadRng) {

        let size = rnd.gen_range(self.particle_size.start..self.particle_size.end);
        let final_size = rnd.gen_range(self.particle_final_size.start..self.particle_final_size.end);

        let particle = particle::Particle {
            position: self.position + Vec3{x: rnd.gen_range(-1.0..=1.0), y: rnd.gen_range(-1.0..=1.0), z: 0.0} * rnd.gen_range(0.0..self.radius),
            alive: true,
            start: false,

            size: Vec2 {x: size, y: size},
            final_size: Vec2 {x: final_size, y: final_size },
            size_change: 0.0,

            velocity: get_rand_vec2(&self.start_velocity, rnd),
            final_velocity: get_rand_vec2(&self.final_velocity, rnd),
            velocity_change: Vec2 { x: 0.0, y: 0.0 },

            direction: get_rand_vec2(&self.start_direction, rnd),
            final_direction: get_rand_vec2(&self.final_direction, rnd),
            direction_change: Vec2 { x: 0.0, y: 0.0 },

            age: 0.0,
            lifespan: rnd.gen_range(self.particle_lifespan.start..self.particle_lifespan.end),

            color: get_rand_vec4(&self.start_color, rnd),
            final_color: get_rand_vec4(&self.final_color, rnd),
            color_change: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
            only_affect_alpha: self.only_affect_alpha,

            mass: rnd.gen_range(self.start_mass.start..self.start_mass.end),
            final_mass: rnd.gen_range(self.final_mass.start..self.final_mass.end),
            mass_change: 0.0,
        };

        self.particles.push(particle);
    }

    pub fn initial_spawn(&mut self, rnd: &mut rand::rngs::ThreadRng) {
        for _ in 0..rnd.gen_range(self.initial_count.start..self.initial_count.end) {
            self.spawn_particle(rnd);
        }
    }
}