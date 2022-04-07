use super::*;

pub struct Damage <'a> {
    pub emmiter: emitter::Emitter,
    mesh: Mesh,
    shader: &'a Shader,
    position: Position
}

impl <'a> Damage <'a> {
    pub fn new(
        shader: &'a Shader
    ) -> Damage {
        let emmiter = emitter::Emitter {
            position: Vec3 {x: 300.0, y: 300.0, z: -1.0},
            radius: 45.0,
            
            age: 0.0,
            spawn_time: 100.0,
            lifespan: 15000.0,
            
            alive: true,
            start: false,
            
            particles: vec![],
            
            particle_size: Range { start: 5.0, end: 8.0 },
            particle_final_size: Range { start: 5.0, end: 8.0 },
            
            particle_lifespan: Range { start: 1000.0, end: 2000.0 },
            
            initial_count: Range { start: 10, end: 30 },
            spawn_count: Range { start: 10, end: 30 },
            
            spawn_interval: Range { start: 50.0, end: 100.0 },
            current_interval: 0.0,
            next_interval: 0.0,
            
            start_color: Range { start: Vec4 {x: 0.9, y: 0.0, z: 0.0, w: 1.0 }, end: Vec4 { x: 1.0, y: 0.1, z: 0.1, w: 1.1 } },
            final_color: Range { start: Vec4 {x: 0.0, y: 0.0, z: 0.0, w: 0.0 }, end: Vec4 { x: 0.01, y: 0.01, z: 0.01, w: 0.1 } },
            only_affect_alpha: true,
            
            start_velocity: Range { start: Vec2 {x: 1.0, y: 1.0}, end: Vec2 { x: 2.0, y: 2.0 } },
            final_velocity: Range { start: Vec2 {x: 0.25, y: 0.25}, end: Vec2 { x: 0.5, y: 0.5 } },
            
            start_direction: Range { start: Vec2 {x: -1.0, y: 1.0}, end: Vec2 { x: 1.0, y: 1.00001 } },
            final_direction: Range { start: Vec2 {x: 0.0, y: 0.0}, end: Vec2 { x: 0.1, y: 0.000001 } },
            
            start_mass: Range { start: 0.0, end: 0.0001 },
            final_mass: Range { start: 5.0, end: 8.0 },
        };

        Damage {
            emmiter: emmiter,
            mesh: Mesh::new(),
            shader: shader,
            position: vec3(0.0, 0.0, -1.0)
        }
    }

    pub fn update(&mut self, engine: &Engine, vp: &Matrix4<f32>) {
        let mut rnd = rand::thread_rng();
        let delta = engine.get_frametime() as f32 / 1_000_000.0;
        self.emmiter.update(&(self.mesh), self.shader, &mut rnd, delta, vp);
    }

    pub fn get_emmiter(&mut self) -> &mut emitter::Emitter {
        &mut (self.emmiter)
    }
}

impl <'a> Particle for Damage <'a> {
    fn render(&mut self, engine: &mut Engine, vp: &Matrix4<f32>) {
        self.update(engine, vp);
    }
}

impl <'a> Positioning for Damage <'a> {
    fn get_size(&self) -> &Size {
        &NO_SIZE
    } 

    fn get_position(&self) -> &Position {
        &(self.position)
    }

    fn set_size(&mut self, _size: &Size) {}

    fn set_position(&mut self, pos: &Position) {
        self.position = pos.clone();
        self.emmiter.position = pos.clone();
    }
}