pub mod main_menu;
pub mod button;
pub mod gameplay;

pub mod game;


use super::*;

use cgmath::{vec2, vec3, Matrix4, vec4};
use glfw::{Key, Action, WindowEvent};

use button::Button;
use main_menu::MainMenu;
use gameplay::Gameplay;
pub use game::Game;

type EventT = Vec<glfw::WindowEvent>;