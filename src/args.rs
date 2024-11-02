use crate::config::Mode;
use crate::rules::RuleSet;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Args {
    /// Rule set to use
    #[arg(long, value_enum, default_value = "conway")]
    pub rules: RuleSet,

    /// Simulation speed in ticks per second
    #[arg(long, default_value = "30.0")]
    pub speed: f64,

    /// Display mode: color or image
    #[arg(long, value_enum, default_value = "color")]
    pub mode: DisplayMode,
}

#[derive(ValueEnum, Clone)]
pub enum DisplayMode {
    Color,
    Image,
}

impl From<DisplayMode> for Mode {
    fn from(display_mode: DisplayMode) -> Self {
        match display_mode {
            DisplayMode::Color => Mode::Color,
            DisplayMode::Image => Mode::Image,
        }
    }
}
