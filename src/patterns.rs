use crate::state::Cell;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Pattern {
    Single,
    Glider,
    Blinker,
    Toad,
    Beacon,
    Pulsar,
    Block,
}

impl Pattern {
    pub fn add_to_grid(&self, cells: &mut [Vec<Cell>], x: usize, y: usize) {
        match self {
            Pattern::Single => add_single(cells, x, y),
            Pattern::Glider => add_glider(cells, x, y),
            Pattern::Blinker => add_blinker(cells, x, y),
            Pattern::Toad => add_toad(cells, x, y),
            Pattern::Beacon => add_beacon(cells, x, y),
            Pattern::Pulsar => add_pulsar(cells, x, y),
            Pattern::Block => add_block(cells, x, y),
        }
    }
}

fn add_single(cells: &mut [Vec<Cell>], x: usize, y: usize) {
    cells[y][x].is_alive = true;
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
