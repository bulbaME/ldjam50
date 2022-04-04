use super::*;

enum GameState {
    Backstory,
    Gameplay
}

pub struct Gameplay {
    paused: bool,
    state: GameState,
    counter: u128   // nanoseconds
}

impl Gameplay {
    pub fn new() -> Gameplay {
        Gameplay {
            paused: false,
            state: GameState::Backstory,
            counter: 0
        }
    }

    pub fn update(&mut self, engine: &mut Engine) {
        self.counter += engine.get_frametime() as u128;
        println!("{:.0}", self.counter as f64 / 1_000_000_000.0);
    }
}