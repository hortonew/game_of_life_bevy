use bevy::prelude::*;
use rand::Rng;

const GRID_WIDTH: usize = 50;
const GRID_HEIGHT: usize = 50;
const CELL_SIZE: f32 = 10.0;
const ALIVE_COLOR: Color = Color::srgb(0.0, 1.0, 0.0); // Green for alive cells
const DEAD_COLOR: Color = Color::srgb(0.0, 0.0, 0.0); // Black for dead cells

#[derive(Resource)]
struct GameState {
    cells: Vec<Vec<Cell>>,      // 2D grid of cells with metadata
    next_cells: Vec<Vec<bool>>, // Temporary grid for next state calculations
}

#[derive(Clone)]
struct Cell {
    is_alive: bool,
    activation_count: u32, // Counter for how many times this cell has been alive
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .insert_resource(GameState {
            cells: generate_random_grid(),
            next_cells: vec![vec![false; GRID_WIDTH]; GRID_HEIGHT],
        })
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (render_cells, update_cells))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 30.0));

    // #[cfg(debug_assertions)]
    // {
    //     use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
    //     use bevy::diagnostic::LogDiagnosticsPlugin;
    //     app.add_plugins(LogDiagnosticsPlugin::default());
    //     app.add_plugins(FrameTimeDiagnosticsPlugin);
    // }

    app.run();
}

// Generate a random initial grid state
fn generate_random_grid() -> Vec<Vec<Cell>> {
    let mut rng = rand::thread_rng();
    (0..GRID_HEIGHT)
        .map(|_| {
            (0..GRID_WIDTH)
                .map(|_| Cell {
                    is_alive: rng.gen_bool(0.2), // 20% chance of cell being alive
                    activation_count: 0,         // Start with 0 activations
                })
                .collect()
        })
        .collect()
}

// Setup function to initialize the camera and create cell entities
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: DEAD_COLOR,
                    custom_size: Some(Vec2::splat(CELL_SIZE)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    x as f32 * CELL_SIZE - GRID_WIDTH as f32 * CELL_SIZE / 2.0,
                    y as f32 * CELL_SIZE - GRID_HEIGHT as f32 * CELL_SIZE / 2.0,
                    0.0,
                ),
                ..Default::default()
            });
        }
    }
}

// Update cells according to Game of Life rules
fn update_cells(mut game_state: ResMut<GameState>) {
    // First pass: determine the next state for each cell
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let alive_neighbors = count_alive_neighbors(&game_state.cells, x, y);
            let is_alive = game_state.cells[y][x].is_alive;

            game_state.next_cells[y][x] = match (is_alive, alive_neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }

    // Second pass: apply the next state and update activation counts
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let next_alive = game_state.next_cells[y][x]; // Store the next state in a temporary variable
            let cell = &mut game_state.cells[y][x];
            if next_alive && !cell.is_alive {
                cell.activation_count += 1; // Increment count if cell becomes alive
                                            // if cell.activation_count % 5 == 0 {
                                            //     println!("Cell at ({}, {}) has been alive {} times", x, y, cell.activation_count);
                                            // }
            }
            cell.is_alive = next_alive;
        }
    }
}

// Count the alive neighbors of a cell at (x, y)
fn count_alive_neighbors(cells: &[Vec<Cell>], x: usize, y: usize) -> usize {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as isize + dx).rem_euclid(GRID_WIDTH as isize) as usize;
            let ny = (y as isize + dy).rem_euclid(GRID_HEIGHT as isize) as usize;
            if cells[ny][nx].is_alive {
                count += 1;
            }
        }
    }
    count
}

// Render the cells according to their current state
fn render_cells(game_state: Res<GameState>, mut query: Query<&mut Sprite>) {
    for (i, mut sprite) in query.iter_mut().enumerate() {
        let x = i % GRID_WIDTH;
        let y = i / GRID_WIDTH;
        sprite.color = if game_state.cells[y][x].is_alive {
            ALIVE_COLOR
        } else {
            DEAD_COLOR
        };
    }
}
