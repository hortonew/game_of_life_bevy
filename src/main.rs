use bevy::prelude::*;
use clap::{Parser, ValueEnum};
use rand::Rng;

const GRID_WIDTH: usize = 50;
const GRID_HEIGHT: usize = 50;
const CELL_SIZE: f32 = 10.0;
const ALIVE_COLOR: Color = Color::srgb(0.0, 1.0, 0.0); // Green for alive cells
const DEAD_COLOR: Color = Color::srgb(0.0, 0.0, 0.0); // Black for dead cells

#[derive(Parser)]
struct Args {
    /// Rule set to use (e.g., conway, highlife, day_and_night, etc.)
    #[arg(long, value_enum, default_value = "conway")]
    rules: RuleSet,

    /// Simulation speed in ticks per second
    #[arg(long, default_value = "30.0")]
    speed: f64,
}

struct Rules {
    survival_counts: Vec<usize>, // Counts of neighbors needed to survive
    birth_counts: Vec<usize>,    // Counts of neighbors needed to be born
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum RuleSet {
    Conway,
    Highlife,
    DayAndNight,
    Seeds,
    LifeWithoutDeath,
    Maze,
    Anneal,
}

impl RuleSet {
    fn to_rules(self) -> Rules {
        match self {
            RuleSet::Conway => Rules::conway(),
            RuleSet::Highlife => Rules::highlife(),
            RuleSet::DayAndNight => Rules::day_and_night(),
            RuleSet::Seeds => Rules::seeds(),
            RuleSet::LifeWithoutDeath => Rules::life_without_death(),
            RuleSet::Maze => Rules::maze(),
            RuleSet::Anneal => Rules::anneal(),
        }
    }
}

impl Rules {
    #[allow(dead_code)]
    fn conway() -> Self {
        Self {
            survival_counts: vec![2, 3],
            birth_counts: vec![3],
        }
    }
    #[allow(dead_code)]
    fn highlife() -> Self {
        Self {
            survival_counts: vec![2, 3],
            birth_counts: vec![3, 6], // Additional birth condition: 6 neighbors
        }
    }
    #[allow(dead_code)]
    fn day_and_night() -> Self {
        Self {
            survival_counts: vec![3, 4, 6, 7, 8],
            birth_counts: vec![3, 6, 7, 8], // Birth and survival counts are similar
        }
    }
    #[allow(dead_code)]
    fn seeds() -> Self {
        Self {
            survival_counts: vec![], // No survival counts; all live cells die
            birth_counts: vec![2],   // Cells are born with exactly 2 neighbors
        }
    }
    #[allow(dead_code)]
    fn life_without_death() -> Self {
        Self {
            survival_counts: vec![1, 2, 3, 4, 5, 6, 7, 8], // Cells stay alive no matter their neighbors
            birth_counts: vec![3],                         // Standard birth condition
        }
    }
    #[allow(dead_code)]
    fn maze() -> Self {
        Self {
            survival_counts: vec![1, 2, 3, 4, 5],
            birth_counts: vec![3],
        }
    }
    #[allow(dead_code)]
    fn anneal() -> Self {
        Self {
            survival_counts: vec![4, 6, 7, 8],
            birth_counts: vec![3, 5, 6, 7, 8],
        }
    }
}

#[derive(Resource)]
struct GameState {
    cells: Vec<Vec<Cell>>,      // 2D grid of cells with metadata
    next_cells: Vec<Vec<bool>>, // Temporary grid for next state calculations
    rules: Rules,
}

#[derive(Clone)]
struct Cell {
    is_alive: bool,
    activation_count: u32, // Counter for how many times this cell has been alive
}

fn main() {
    let args = Args::parse();
    let rules = args.rules.to_rules();
    let tick_duration = if args.speed != 1.0 { 1.0 / args.speed } else { 1.0 };

    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .insert_resource(GameState {
            // cells: generate_random_grid(),
            cells: generate_empty_grid(),
            next_cells: vec![vec![false; GRID_WIDTH]; GRID_HEIGHT],
            rules,
        })
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (render_cells, update_cells))
        .insert_resource(Time::<Fixed>::from_seconds(tick_duration));

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
#[allow(dead_code)]
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

#[allow(dead_code)]
fn generate_empty_grid() -> Vec<Vec<Cell>> {
    (0..GRID_HEIGHT)
        .map(|_| {
            (0..GRID_WIDTH)
                .map(|_| Cell {
                    is_alive: false,
                    activation_count: 0,
                })
                .collect()
        })
        .collect()
}

// Setup function to initialize the camera and create cell entities
fn setup(mut commands: Commands, mut game_state: ResMut<GameState>) {
    commands.spawn(Camera2dBundle::default());

    // Add a glider and a blinker to the initial grid
    let patterns = vec![
        // (Pattern::Glider, 5, 5),
        // (Pattern::Blinker, 20, 20),
        // (Pattern::Toad, 15, 15),
        // (Pattern::Beacon, 30, 10),
        (Pattern::Pulsar, 10, 30),
        // (Pattern::Block, 35, 35),
    ];
    for (pattern, x, y) in patterns {
        pattern.add_to_grid(&mut game_state.cells, x, y);
    }

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
    let rules = &game_state.rules;

    // Temporary storage for the next state to avoid mutable borrowing conflicts
    let mut new_next_cells = vec![vec![false; GRID_WIDTH]; GRID_HEIGHT];

    // First pass: determine the next state for each cell
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let alive_neighbors = count_alive_neighbors(&game_state.cells, x, y);
            let is_alive = game_state.cells[y][x].is_alive;

            new_next_cells[y][x] = if is_alive {
                rules.survival_counts.contains(&alive_neighbors)
            } else {
                rules.birth_counts.contains(&alive_neighbors)
            };
        }
    }

    // Second pass: update game_state.next_cells with the calculated next state
    game_state.next_cells = new_next_cells;

    // Third pass: apply the next state and update activation counts
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let next_alive = game_state.next_cells[y][x];
            let cell = &mut game_state.cells[y][x];
            if next_alive && !cell.is_alive {
                cell.activation_count += 1; // Increment count if cell becomes alive
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

#[allow(dead_code)]
enum Pattern {
    Glider,
    Blinker,
    Toad,
    Beacon,
    Pulsar,
    Block,
}

impl Pattern {
    fn add_to_grid(&self, cells: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
        match self {
            Pattern::Glider => add_glider(cells, x, y),
            Pattern::Blinker => add_blinker(cells, x, y),
            Pattern::Toad => add_toad(cells, x, y),
            Pattern::Beacon => add_beacon(cells, x, y),
            Pattern::Pulsar => add_pulsar(cells, x, y),
            Pattern::Block => add_block(cells, x, y),
        }
    }
}

fn add_glider(cells: &mut [Vec<Cell>], x: usize, y: usize) {
    // Coordinates for a glider pattern
    let glider_coords = [(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];
    for (dx, dy) in glider_coords {
        cells[y + dy][x + dx].is_alive = true;
    }
}

fn add_blinker(cells: &mut [Vec<Cell>], x: usize, y: usize) {
    // Coordinates for a blinker pattern
    let blinker_coords = [(0, 1), (1, 1), (2, 1)];
    for (dx, dy) in blinker_coords {
        cells[y + dy][x + dx].is_alive = true;
    }
}

fn add_toad(cells: &mut [Vec<Cell>], x: usize, y: usize) {
    let toad_coords = [(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)];
    for (dx, dy) in toad_coords {
        cells[y + dy][x + dx].is_alive = true;
    }
}

fn add_beacon(cells: &mut [Vec<Cell>], x: usize, y: usize) {
    let beacon_coords = [(0, 0), (1, 0), (0, 1), (1, 1), (2, 2), (3, 2), (2, 3), (3, 3)];
    for (dx, dy) in beacon_coords {
        cells[y + dy][x + dx].is_alive = true;
    }
}

fn add_pulsar(cells: &mut [Vec<Cell>], x: usize, y: usize) {
    let pulsar_coords = [
        (2, 0),
        (3, 0),
        (4, 0),
        (8, 0),
        (9, 0),
        (10, 0),
        (0, 2),
        (5, 2),
        (7, 2),
        (12, 2),
        (0, 3),
        (5, 3),
        (7, 3),
        (12, 3),
        (0, 4),
        (5, 4),
        (7, 4),
        (12, 4),
        (2, 5),
        (3, 5),
        (4, 5),
        (8, 5),
        (9, 5),
        (10, 5),
        (2, 7),
        (3, 7),
        (4, 7),
        (8, 7),
        (9, 7),
        (10, 7),
        (0, 8),
        (5, 8),
        (7, 8),
        (12, 8),
        (0, 9),
        (5, 9),
        (7, 9),
        (12, 9),
        (0, 10),
        (5, 10),
        (7, 10),
        (12, 10),
        (2, 12),
        (3, 12),
        (4, 12),
        (8, 12),
        (9, 12),
        (10, 12),
    ];
    for (dx, dy) in pulsar_coords {
        cells[y + dy][x + dx].is_alive = true;
    }
}

fn add_block(cells: &mut [Vec<Cell>], x: usize, y: usize) {
    let block_coords = [(0, 0), (1, 0), (0, 1), (1, 1)];
    for (dx, dy) in block_coords {
        cells[y + dy][x + dx].is_alive = true;
    }
}
