use bevy::prelude::*;
use clap::Parser;
mod args;
mod config;
mod patterns;
mod rules;
mod state;
mod systems;

use bevy_embedded_assets::EmbeddedAssetPlugin;
use rand::Rng;

fn main() {
    let args = args::Args::parse();
    let mode = config::Mode::from(args.mode);
    let tick_duration = if args.speed != 1.0 { 1.0 / args.speed } else { 1.0 };

    App::new()
        .add_plugins((EmbeddedAssetPlugin::default(), DefaultPlugins))
        .insert_resource(state::GameState {
            cells: generate_empty_grid(),
            next_cells: vec![vec![false; config::GRID_WIDTH]; config::GRID_HEIGHT],
            mode,
            selected_pattern: patterns::Pattern::Glider,
            selected_rules: args.rules,
        })
        .add_systems(Startup, systems::setup)
        .add_systems(FixedUpdate, (systems::render_cells, systems::render_images))
        .add_systems(
            Update,
            (
                systems::trigger_selected_pattern,
                systems::change_selected_pattern,
                systems::change_selected_rules,
                systems::update_selected_pattern_text,
                systems::update_selected_rules_text,
                systems::kill_all_cells,
                systems::update_cells,
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
