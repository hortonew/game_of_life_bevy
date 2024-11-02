use crate::state::Cell;
use crate::{config, patterns::Pattern, state::GameState};
use bevy::prelude::*;

pub fn setup(mut commands: Commands, mut game_state: ResMut<GameState>, asset_server: Res<AssetServer>) {
    // Spawn the 2D camera
    commands.spawn(Camera2dBundle::default());

    // Load textures for alive and dead cells
    let textures = Textures {
        alive_texture: asset_server.load(config::ALIVE_IMAGE),
        dead_texture: asset_server.load(config::DEAD_IMAGE),
    };
    commands.insert_resource(textures.clone());

    // Initialize the grid pattern
    let patterns = vec![(Pattern::Pulsar, 10, 30)];
    for (pattern, x, y) in patterns {
        pattern.add_to_grid(&mut game_state.cells, x, y);
    }

    // Spawn a grid of sprites, either using color or texture mode based on the config
    for y in 0..crate::config::GRID_HEIGHT {
        for x in 0..crate::config::GRID_WIDTH {
            if config::USE_COLOR_MODE {
                // Color mode: spawn with color
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: config::DEAD_COLOR,
                        custom_size: Some(Vec2::splat(config::CELL_SIZE)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        x as f32 * config::CELL_SIZE - config::GRID_WIDTH as f32 * config::CELL_SIZE / 2.0,
                        y as f32 * config::CELL_SIZE - config::GRID_HEIGHT as f32 * config::CELL_SIZE / 2.0,
                        0.0,
                    ),
                    ..Default::default()
                });
            } else {
                // Image mode: spawn with texture
                commands.spawn(SpriteBundle {
                    texture: textures.dead_texture.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(config::CELL_SIZE)), // Limit sprite to cell size
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        x as f32 * config::CELL_SIZE - config::GRID_WIDTH as f32 * config::CELL_SIZE / 2.0,
                        y as f32 * config::CELL_SIZE - config::GRID_HEIGHT as f32 * config::CELL_SIZE / 2.0,
                        0.0,
                    ),
                    ..Default::default()
                });
            }
        }
    }
}

pub fn update_cells(mut game_state: ResMut<GameState>) {
    let rules = &game_state.rules;

    // Temporary storage for the next state to avoid mutable borrowing conflicts
    let mut new_next_cells = vec![vec![false; config::GRID_WIDTH]; config::GRID_HEIGHT];

    // First pass: determine the next state for each cell
    for (y, row) in new_next_cells.iter_mut().enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            let alive_neighbors = count_alive_neighbors(&game_state.cells, x, y);
            let is_alive = game_state.cells[y][x].is_alive;

            *cell = if is_alive {
                rules.survival_counts.contains(&alive_neighbors)
            } else {
                rules.birth_counts.contains(&alive_neighbors)
            };
        }
    }

    // Second pass: update game_state.next_cells with the calculated next state
    game_state.next_cells = new_next_cells;

    // Third pass: apply the next state and update activation counts
    for y in 0..config::GRID_HEIGHT {
        for x in 0..config::GRID_WIDTH {
            let next_alive = game_state.next_cells[y][x];
            let cell = &mut game_state.cells[y][x];
            if next_alive && !cell.is_alive {
                cell.activation_count += 1; // Increment count if cell becomes alive
            }
            cell.is_alive = next_alive;
        }
    }
}

pub fn render_cells(game_state: Res<GameState>, mut query: Query<&mut Sprite>) {
    if config::USE_COLOR_MODE {
        for (i, mut sprite) in query.iter_mut().enumerate() {
            let x = i % config::GRID_WIDTH;
            let y = i / config::GRID_WIDTH;
            sprite.color = if game_state.cells[y][x].is_alive {
                config::ALIVE_COLOR
            } else {
                config::DEAD_COLOR
            };
        }
    }
}

fn count_alive_neighbors(cells: &[Vec<Cell>], x: usize, y: usize) -> usize {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as isize + dx).rem_euclid(config::GRID_WIDTH as isize) as usize;
            let ny = (y as isize + dy).rem_euclid(config::GRID_HEIGHT as isize) as usize;
            if cells[ny][nx].is_alive {
                count += 1;
            }
        }
    }
    count
}

#[derive(Resource, Clone)]
pub struct Textures {
    alive_texture: Handle<Image>,
    dead_texture: Handle<Image>,
}

pub fn render_images(game_state: Res<GameState>, textures: Res<Textures>, mut query: Query<&mut Handle<Image>>) {
    if !config::USE_COLOR_MODE {
        for (i, mut texture_handle) in query.iter_mut().enumerate() {
            let x = i % config::GRID_WIDTH;
            let y = i / config::GRID_WIDTH;

            // Set the texture based on the cell state
            *texture_handle = if game_state.cells[y][x].is_alive {
                textures.alive_texture.clone()
            } else {
                textures.dead_texture.clone()
            };
        }
    }
}
