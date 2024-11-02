use crate::state::Cell;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Pattern {
    Single,
    Glider,
    Blinker,
    Toad,
    Beacon,
    Pulsar,
    Block,
    LightweightSpaceship,
    MiddleweightSpaceship,
    HeavyweightSpaceship,
    PentaDecathlon,
    Clock,
    Beehive,
    Loaf,
    Boat,
    Tub,
    GosperGliderGun,
    Diehard,
    Acorn,
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
            Pattern::LightweightSpaceship => add_lightweight_spaceship(cells, x, y),
            Pattern::MiddleweightSpaceship => add_middleweight_spaceship(cells, x, y),
            Pattern::HeavyweightSpaceship => add_heavyweight_spaceship(cells, x, y),
            Pattern::PentaDecathlon => add_penta_decathlon(cells, x, y),
            Pattern::Clock => add_clock(cells, x, y),
            Pattern::Beehive => add_beehive(cells, x, y),
            Pattern::Loaf => add_loaf(cells, x, y),
            Pattern::Boat => add_boat(cells, x, y),
            Pattern::Tub => add_tub(cells, x, y),
            Pattern::GosperGliderGun => add_gosper_glider_gun(cells, x, y),
            Pattern::Diehard => add_diehard(cells, x, y),
            Pattern::Acorn => add_acorn(cells, x, y),
        }
    }

    pub fn next(&self) -> Pattern {
        use Pattern::*;
        match self {
            Single => Glider,
            Glider => Blinker,
            Blinker => Toad,
            Toad => Beacon,
            Beacon => Pulsar,
            Pulsar => Block,
            Block => LightweightSpaceship,
            LightweightSpaceship => MiddleweightSpaceship,
            MiddleweightSpaceship => HeavyweightSpaceship,
            HeavyweightSpaceship => PentaDecathlon,
            PentaDecathlon => Clock,
            Clock => Beehive,
            Beehive => Loaf,
            Loaf => Boat,
            Boat => Tub,
            Tub => GosperGliderGun,
            GosperGliderGun => Diehard,
            Diehard => Acorn,
            Acorn => Single, // Wrap around to the first pattern
        }
    }

    pub fn previous(&self) -> Pattern {
        use Pattern::*;
        match self {
            Single => Acorn, // Wrap around to the last pattern
            Glider => Single,
            Blinker => Glider,
            Toad => Blinker,
            Beacon => Toad,
            Pulsar => Beacon,
            Block => Pulsar,
            LightweightSpaceship => Block,
            MiddleweightSpaceship => LightweightSpaceship,
            HeavyweightSpaceship => MiddleweightSpaceship,
            PentaDecathlon => HeavyweightSpaceship,
            Clock => PentaDecathlon,
            Beehive => Clock,
            Loaf => Beehive,
            Boat => Loaf,
            Tub => Boat,
            GosperGliderGun => Tub,
            Diehard => GosperGliderGun,
            Acorn => Diehard,
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

fn add_lightweight_spaceship(cells: &mut [Vec<Cell>], x: usize, y: usize) {
    let lwss_coords = [(0, 1), (3, 1), (4, 2), (0, 3), (4, 3), (1, 4), (2, 4), (3, 4)];
    for (dx, dy) in lwss_coords {
        cells[y + dy][x + dx].is_alive = true;
    }
}

fn add_middleweight_spaceship(cells: &mut [Vec<Cell>], x: usize, y: usize) {
    let mwss_coords = [(0, 1), (4, 1), (5, 2), (0, 3), (5, 3), (1, 4), (2, 4), (3, 4), (4, 4)];
    for (dx, dy) in mwss_coords {
        cells[y + dy][x + dx].is_alive = true;
    }
}

fn add_heavyweight_spaceship(cells: &mut [Vec<Cell>], x: usize, y: usize) {
    let hwss_coords = [
        (0, 1),
        (5, 1),
        (6, 2),
        (0, 3),
        (6, 3),
        (1, 4),
        (2, 4),
        (3, 4),
        (4, 4),
        (5, 4),
    ];
    for (dx, dy) in hwss_coords {
        cells[y + dy][x + dx].is_alive = true;
    }
}

fn add_penta_decathlon(cells: &mut [Vec<Cell>], x: usize, y: usize) {
    let penta_decathlon_coords = [
        (1, 0),
        (1, 1),
        (1, 2),
        (1, 3),
        (1, 5),
        (1, 6),
        (1, 7),
        (1, 8),
        (0, 4),
        (2, 4),
    ];
    for (dx, dy) in penta_decathlon_coords {
        cells[y + dy][x + dx].is_alive = true;
    }
}

fn add_clock(cells: &mut [Vec<Cell>], x: usize, y: usize) {
    let clock_coords = [(1, 0), (2, 0), (0, 1), (3, 1), (0, 2), (3, 2), (1, 3), (2, 3)];
    for (dx, dy) in clock_coords {
        cells[y + dy][x + dx].is_alive = true;
    }
}

fn add_beehive(cells: &mut [Vec<Cell>], x: usize, y: usize) {
    let beehive_coords = [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)];
    for (dx, dy) in beehive_coords {
        cells[y + dy][x + dx].is_alive = true;
    }
}

fn add_loaf(cells: &mut [Vec<Cell>], x: usize, y: usize) {
    let loaf_coords = [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)];
    for (dx, dy) in loaf_coords {
        cells[y + dy][x + dx].is_alive = true;
    }
}

fn add_boat(cells: &mut [Vec<Cell>], x: usize, y: usize) {
    let boat_coords = [(0, 0), (1, 0), (2, 1), (0, 1), (1, 2)];
    for (dx, dy) in boat_coords {
        cells[y + dy][x + dx].is_alive = true;
    }
}

fn add_tub(cells: &mut [Vec<Cell>], x: usize, y: usize) {
    let tub_coords = [(1, 0), (0, 1), (2, 1), (1, 2)];
    for (dx, dy) in tub_coords {
        cells[y + dy][x + dx].is_alive = true;
    }
}

fn add_gosper_glider_gun(cells: &mut [Vec<Cell>], x: usize, y: usize) {
    let gosper_glider_gun_coords = [
        (0, 4),
        (1, 4),
        (0, 5),
        (1, 5),
        (10, 4),
        (10, 5),
        (10, 6),
        (11, 3),
        (11, 7),
        (12, 2),
        (12, 8),
        (13, 2),
        (13, 8),
        (14, 5),
        (15, 3),
        (15, 7),
        (16, 4),
        (16, 5),
        (16, 6),
        (17, 5),
        (20, 2),
        (20, 3),
        (20, 4),
        (21, 2),
        (21, 3),
        (21, 4),
        (22, 1),
        (22, 5),
        (24, 0),
        (24, 1),
        (24, 5),
        (24, 6),
        (34, 2),
        (34, 3),
        (35, 2),
        (35, 3),
    ];

    let max_x = cells[0].len();
    let max_y = cells.len();

    for (dx, dy) in gosper_glider_gun_coords {
        let new_x = x + dx;
        let new_y = y + dy;

        if new_x < max_x && new_y < max_y {
            cells[new_y][new_x].is_alive = true;
        }
    }
}

fn add_diehard(cells: &mut [Vec<Cell>], x: usize, y: usize) {
    let diehard_coords = [(0, 1), (1, 1), (1, 2), (5, 2), (6, 2), (7, 2), (6, 0)];
    for (dx, dy) in diehard_coords {
        cells[y + dy][x + dx].is_alive = true;
    }
}

fn add_acorn(cells: &mut [Vec<Cell>], x: usize, y: usize) {
    let acorn_coords = [(1, 0), (3, 1), (0, 2), (1, 2), (4, 2), (5, 2), (6, 2)];
    for (dx, dy) in acorn_coords {
        cells[y + dy][x + dx].is_alive = true;
    }
}
