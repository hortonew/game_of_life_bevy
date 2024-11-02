use crate::{config::Mode, rules::Rules};
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct GameState {
    pub cells: Vec<Vec<Cell>>,
    pub next_cells: Vec<Vec<bool>>,
    pub rules: Rules,
    pub mode: Mode,
}

#[derive(Clone)]
pub struct Cell {
    pub is_alive: bool,
    pub activation_count: u32,
}
