use bevy::prelude::*;
use rand::Rng;

const GRID_WIDTH: usize = 50; // Width of the grid
const GRID_HEIGHT: usize = 50; // Height of the grid
const CELL_SIZE: f32 = 10.0; // Size of each cell in pixels
const ALIVE_COLOR: Color = Color::srgb(0.0, 1.0, 0.0); // Green for alive cells
const DEAD_COLOR: Color = Color::srgb(0.0, 0.0, 0.0); // Black for dead cells

// Game state resource holding cell information
#[derive(Resource)]
struct GameState {
    cells: Vec<Vec<bool>>,      // 2D grid of cells
    next_cells: Vec<Vec<bool>>, // Temporary grid for next state calculations
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameState {
            cells: generate_random_grid(),
            next_cells: vec![vec![false; GRID_WIDTH]; GRID_HEIGHT],
        })
        .add_systems(Startup, setup) // Use add_systems(Startup, ...) for setup systems
        .add_systems(Update, (update_cells, render_cells)) // Use add_systems(Update, ...) for update systems
        .run();
}

// Generate a random initial grid state
fn generate_random_grid() -> Vec<Vec<bool>> {
    let mut rng = rand::thread_rng();
    (0..GRID_HEIGHT)
        .map(|_| {
            (0..GRID_WIDTH)
                .map(|_| rng.gen_bool(0.2)) // 20% chance of cell being alive
                .collect()
        })
        .collect()
}

// Setup function to initialize the camera and create cell entities
fn setup(mut commands: Commands) {
    // Add a 2D camera
    commands.spawn(Camera2dBundle::default());

    // Spawn each cell as an entity with a sprite component
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
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let alive_neighbors = count_alive_neighbors(&game_state.cells, x, y);
            let is_alive = game_state.cells[y][x];
            game_state.next_cells[y][x] = match (is_alive, alive_neighbors) {
                (true, 2) | (true, 3) => true, // Stays alive
                (false, 3) => true,            // Becomes alive
                _ => false,                    // Dies or stays dead
            };
        }
    }

    // Manually copy `next_cells` back into `cells`
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            game_state.cells[y][x] = game_state.next_cells[y][x];
        }
    }
}

// Count the alive neighbors of a cell at (x, y)
fn count_alive_neighbors(cells: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as isize + dx).rem_euclid(GRID_WIDTH as isize) as usize;
            let ny = (y as isize + dy).rem_euclid(GRID_HEIGHT as isize) as usize;
            if cells[ny][nx] {
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
        sprite.color = if game_state.cells[y][x] {
            ALIVE_COLOR
        } else {
            DEAD_COLOR
        };
    }
}
