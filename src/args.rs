use crate::rules::RuleSet;
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Rule set to use
    #[arg(long, value_enum, default_value = "conway")]
    pub rules: RuleSet,

    /// Simulation speed in ticks per second
    #[arg(long, default_value = "30.0")]
    pub speed: f64,
}
