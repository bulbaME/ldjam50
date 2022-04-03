pub mod engine;

use engine::*;

pub enum MainMenu {
    Main,
    Settings,
    Credits
}

pub enum State {
    Game,
    MainMenu(MainMenu)
}