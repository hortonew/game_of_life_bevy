use crate::config::Mode;
use crate::state::{Cell, SelectedPatternText, SelectedRulesText, Textures};
use crate::{config, state::GameState};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rayon::prelude::*;

pub fn setup(mut commands: Commands, game_state: ResMut<GameState>, asset_server: Res<AssetServer>) {
    // Spawn the 2D camera
    commands.spawn(Camera2dBundle::default());

    // Load textures for alive and dead cells
    let textures = Textures {
        alive_texture: asset_server.load(config::ALIVE_IMAGE),
        dead_texture: asset_server.load(config::DEAD_IMAGE),
    };
    commands.insert_resource(textures.clone());

    // Spawn a grid of sprites, either using color or texture mode based on the config
    for y in 0..crate::config::GRID_HEIGHT {
        for x in 0..crate::config::GRID_WIDTH {
            if game_state.mode == Mode::Color {
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
    commands.spawn((
        TextBundle::from_section(
            format!("Selected Pattern: {:?}", game_state.selected_pattern),
            TextStyle {
                font: asset_server.load(config::FONT),
                font_size: 24.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            width: Val::Px(200.0),
            height: Val::Px(50.0),
            right: Val::Px(10.0),
            bottom: Val::Px(10.0),
            ..Default::default()
        }),
        SelectedPatternText, // Marker component
    ));

    commands.spawn((
        TextBundle::from_section(
            format!("Rules: {:?}", game_state.selected_rules),
            TextStyle {
                font: asset_server.load(config::FONT),
                font_size: 24.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            width: Val::Px(200.0),
            height: Val::Px(50.0),
            right: Val::Px(10.0),
            bottom: Val::Px(100.0),
            ..Default::default()
        }),
        SelectedRulesText, // Marker component
    ));
}

pub fn update_cells(mut game_state: ResMut<GameState>) {
    let rules = &game_state.selected_rules.to_rules();

    // Temporary storage for the next state to avoid mutable borrowing conflicts
    let mut new_next_cells = vec![vec![false; config::GRID_WIDTH]; config::GRID_HEIGHT];

    // First pass: determine the next state for each cell in parallel
    new_next_cells.par_iter_mut().enumerate().for_each(|(y, row)| {
        row.iter_mut().enumerate().for_each(|(x, cell)| {
            let alive_neighbors = count_alive_neighbors(&game_state.cells, x, y);
            let is_alive = game_state.cells[y][x].is_alive;

            *cell = if is_alive {
                rules.survival_counts.contains(&alive_neighbors)
            } else {
                rules.birth_counts.contains(&alive_neighbors)
            };
        });
    });

    // Second pass: update game_state.next_cells with the calculated next state
    game_state.next_cells = new_next_cells;

    // Third pass: apply the next state and update activation counts in parallel
    let next_cells = game_state.next_cells.clone();
    game_state.cells.par_iter_mut().enumerate().for_each(|(y, row)| {
        row.iter_mut().enumerate().for_each(|(x, cell)| {
            let next_alive = next_cells[y][x];
            if next_alive && !cell.is_alive {
                cell.activation_count += 1; // Increment count if cell becomes alive
            }
            cell.is_alive = next_alive;
        });
    });
}

pub fn render_cells(game_state: Res<GameState>, mut query: Query<&mut Sprite>) {
    if game_state.mode == Mode::Color {
        let mut sprites: Vec<_> = query.iter_mut().collect();
        sprites.par_iter_mut().enumerate().for_each(|(i, sprite)| {
            let x = i % config::GRID_WIDTH;
            let y = i / config::GRID_WIDTH;
            sprite.color = if game_state.cells[y][x].is_alive {
                config::ALIVE_COLOR
            } else {
                config::DEAD_COLOR
            };
        });
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

pub fn render_images(game_state: Res<GameState>, textures: Res<Textures>, mut query: Query<&mut Handle<Image>>) {
    if game_state.mode == Mode::Image {
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

pub fn trigger_selected_pattern(
    mut game_state: ResMut<GameState>,
    buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Ok(window) = q_windows.get_single() {
            if let Some(cursor_pos) = window.cursor_position() {
                // Calculate the grid offset to position the grid center at the window center
                let grid_offset_x = (config::GRID_WIDTH as f32 * config::CELL_SIZE) / 2.0;
                let grid_offset_y = (config::GRID_HEIGHT as f32 * config::CELL_SIZE) / 2.0;

                // Adjust cursor position relative to the grid center
                let adjusted_x = cursor_pos.x - window.width() / 2.0 + grid_offset_x;
                let adjusted_y = (window.height() - cursor_pos.y) - window.height() / 2.0 + grid_offset_y;

                // Convert to grid coordinates
                let grid_x = (adjusted_x / config::CELL_SIZE) as isize;
                let grid_y = (adjusted_y / config::CELL_SIZE) as isize;

                // Clamp grid coordinates to be within bounds
                let grid_x = grid_x.clamp(0, config::GRID_WIDTH as isize - 1);
                let grid_y = grid_y.clamp(0, config::GRID_HEIGHT as isize - 1);

                // Add the blinker pattern at the clamped grid position
                let selected_pattern = game_state.selected_pattern;
                selected_pattern.add_to_grid(&mut game_state.cells, grid_x as usize, grid_y as usize);
            }
        }
    }
}

pub fn change_selected_pattern(mut game_state: ResMut<GameState>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::KeyD) {
        game_state.selected_pattern = game_state.selected_pattern.next();
    } else if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::KeyA) {
        game_state.selected_pattern = game_state.selected_pattern.previous();
    }
}

pub fn change_selected_rules(mut game_state: ResMut<GameState>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        game_state.selected_rules = game_state.selected_rules.next();
    } else if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        game_state.selected_rules = game_state.selected_rules.previous();
    }
}

pub fn update_selected_pattern_text(
    game_state: Res<GameState>,
    mut query: Query<&mut Text, With<SelectedPatternText>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = format!("Selected Pattern: {:?}", game_state.selected_pattern);
    }
}

pub fn update_selected_rules_text(game_state: Res<GameState>, mut query: Query<&mut Text, With<SelectedRulesText>>) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = format!("Rules: {:?}", game_state.selected_rules);
    }
}

pub fn kill_all_cells(mut game_state: ResMut<GameState>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        for y in 0..config::GRID_HEIGHT {
            for x in 0..config::GRID_WIDTH {
                game_state.cells[y][x].is_alive = false;
            }
        }
    }
}
