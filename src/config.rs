use bevy::prelude::Color;

pub const GRID_WIDTH: usize = 50;
pub const GRID_HEIGHT: usize = 50;
pub const CELL_SIZE: f32 = 10.0;
pub const ALIVE_COLOR: Color = Color::srgb(0.0, 1.0, 0.0); // Green for alive cells
pub const DEAD_COLOR: Color = Color::srgb(0.0, 0.0, 0.0); // Black for dead cells

pub const ALIVE_IMAGE: &str = "frog.png";
pub const DEAD_IMAGE: &str = "empty.png";
pub const USE_COLOR_MODE: bool = false; // Set to `false` for image mode, `true` for color mode
