use bevy::prelude::*;
use clap::Parser;
mod args;
mod config;
mod patterns;
mod rules;
mod state;
mod systems;
use args::Args;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use config::Mode;
use rand::Rng;
use state::GameState;
use systems::{
    change_selected_pattern, kill_all_cells, render_cells, render_images, setup, trigger_selected_pattern,
    update_cells, update_text,
};

fn main() {
    let args = Args::parse();
    let rules = args.rules.to_rules();
    let mode = Mode::from(args.mode);
    let tick_duration = if args.speed != 1.0 { 1.0 / args.speed } else { 1.0 };

    App::new()
        .add_plugins((EmbeddedAssetPlugin::default(), DefaultPlugins))
        .insert_resource(GameState {
            cells: generate_empty_grid(),
            next_cells: vec![vec![false; config::GRID_WIDTH]; config::GRID_HEIGHT],
            rules,
            mode,
            selected_pattern: patterns::Pattern::Glider,
        })
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (render_cells, render_images, update_cells))
        .add_systems(
            Update,
            (
                trigger_selected_pattern,
                change_selected_pattern,
                update_text,
                kill_all_cells,
            ),
        )
        .insert_resource(Time::<Fixed>::from_seconds(tick_duration))
        .run();
}

// Generate a random initial grid state
#[allow(dead_code)]
fn generate_random_grid() -> Vec<Vec<state::Cell>> {
    let mut rng = rand::thread_rng();
    (0..config::GRID_HEIGHT)
        .map(|_| {
            (0..config::GRID_WIDTH)
                .map(|_| state::Cell {
                    is_alive: rng.gen_bool(0.2), // 20% chance of cell being alive
                    activation_count: 0,
                })
                .collect()
        })
        .collect()
}

#[allow(dead_code)]
fn generate_empty_grid() -> Vec<Vec<state::Cell>> {
    (0..config::GRID_HEIGHT)
        .map(|_| {
            (0..config::GRID_WIDTH)
                .map(|_| state::Cell {
                    is_alive: false,
                    activation_count: 0,
                })
                .collect()
        })
        .collect()
}
