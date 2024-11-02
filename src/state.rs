use crate::{config::Mode, patterns::Pattern, rules::Rules};
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct GameState {
    pub cells: Vec<Vec<Cell>>,
    pub next_cells: Vec<Vec<bool>>,
    pub rules: Rules,
    pub mode: Mode,
    pub selected_pattern: Pattern,
}

#[derive(Clone)]
pub struct Cell {
    pub is_alive: bool,
    pub activation_count: u32,
}
