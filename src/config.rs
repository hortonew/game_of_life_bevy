use bevy::prelude::Color;

pub const GRID_WIDTH: usize = 250;
pub const GRID_HEIGHT: usize = 250;
pub const CELL_SIZE: f32 = 15.0;
pub const ALIVE_COLOR: Color = Color::srgb(0.0, 1.0, 0.0); // Green for alive cells
pub const DEAD_COLOR: Color = Color::srgb(0.0, 0.0, 0.0); // Black for dead cells

pub const ALIVE_IMAGE: &str = "embedded://ferris.png";
pub const DEAD_IMAGE: &str = "embedded://empty.png";
pub const FONT: &str = "embedded://fonts/FiraSans-Bold.ttf";

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Color,
    Image,
}
