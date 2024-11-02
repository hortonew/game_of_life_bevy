use clap::ValueEnum;

pub struct Rules {
    pub survival_counts: Vec<usize>,
    pub birth_counts: Vec<usize>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum RuleSet {
    Conway,
    Highlife,
    DayAndNight,
    Seeds,
    LifeWithoutDeath,
    Maze,
    Anneal,
    Diamoeba,
    TwoByTwo,
    Morley,
    Replicator,
    Fredkin,
    Stains,
}

impl RuleSet {
    pub fn to_rules(self) -> Rules {
        match self {
            RuleSet::Conway => Rules::conway(),
            RuleSet::Highlife => Rules::highlife(),
            RuleSet::DayAndNight => Rules::day_and_night(),
            RuleSet::Seeds => Rules::seeds(),
            RuleSet::LifeWithoutDeath => Rules::life_without_death(),
            RuleSet::Maze => Rules::maze(),
            RuleSet::Anneal => Rules::anneal(),
            RuleSet::Diamoeba => Rules::diamoeba(),
            RuleSet::TwoByTwo => Rules::two_by_two(),
            RuleSet::Morley => Rules::morley(),
            RuleSet::Replicator => Rules::replicator(),
            RuleSet::Fredkin => Rules::fredkin(),
            RuleSet::Stains => Rules::stains(),
        }
    }

    pub fn next(&self) -> RuleSet {
        use RuleSet::*;
        match self {
            Conway => Highlife,
            Highlife => DayAndNight,
            DayAndNight => Seeds,
            Seeds => LifeWithoutDeath,
            LifeWithoutDeath => Maze,
            Maze => Anneal,
            Anneal => Diamoeba,
            Diamoeba => TwoByTwo,
            TwoByTwo => Morley,
            Morley => Replicator,
            Replicator => Fredkin,
            Fredkin => Stains,
            Stains => Conway, // Wrap around to the first pattern
        }
    }

    pub fn previous(&self) -> RuleSet {
        use RuleSet::*;
        match self {
            Conway => Stains, // Wrap around to the last pattern
            Highlife => Conway,
            DayAndNight => Highlife,
            Seeds => DayAndNight,
            LifeWithoutDeath => Seeds,
            Maze => LifeWithoutDeath,
            Anneal => Maze,
            Diamoeba => Anneal,
            TwoByTwo => Diamoeba,
            Morley => TwoByTwo,
            Replicator => Morley,
            Fredkin => Replicator,
            Stains => Fredkin,
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
    fn diamoeba() -> Self {
        Self {
            survival_counts: vec![5, 6, 7, 8],
            birth_counts: vec![3, 5, 6, 7, 8],
        }
    }
    fn two_by_two() -> Self {
        Self {
            survival_counts: vec![1, 2, 5],
            birth_counts: vec![3, 6],
        }
    }

    fn morley() -> Self {
        Self {
            survival_counts: vec![2, 4, 5],
            birth_counts: vec![3, 6, 8],
        }
    }
    fn replicator() -> Self {
        Self {
            survival_counts: vec![1, 3, 5, 7],
            birth_counts: vec![1, 3, 5, 7],
        }
    }
    fn fredkin() -> Self {
        Self {
            survival_counts: vec![0, 2, 4, 6, 8],
            birth_counts: vec![1, 3, 5, 7],
        }
    }
    fn stains() -> Self {
        Self {
            survival_counts: vec![2, 3, 5, 6],
            birth_counts: vec![3, 6, 7, 8],
        }
    }
}
