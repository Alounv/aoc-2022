use std::collections::HashSet;

pub mod input;

const TIME: u32 = 32;

/*
const EXAMPLE: &str = "
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";
*/

#[derive(Debug, Clone)]
struct Bl {
    ore: u32,
    clay: u32,
    obsidian: (u32, u32), // ore and clay
    geode: (u32, u32),    // ore and obsidian
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Re {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Ro {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct State {
    resources: Re,
    robots: Ro,
}

fn get_blueprints() -> Vec<Bl> {
    input::INPUT
        //EXAMPLE
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let parts = l.split(" ").collect::<Vec<&str>>();
            Bl {
                ore: parts[6].parse().unwrap(),
                clay: parts[12].parse().unwrap(),
                obsidian: (parts[18].parse().unwrap(), parts[21].parse().unwrap()),
                geode: (parts[27].parse().unwrap(), parts[30].parse().unwrap()),
            }
        })
        .collect()
}

fn adds_steps(state: &State, bp: &Bl, next_steps: &mut HashSet<State>) {
    let previous_resources = state.resources.clone();
    let future_resources = Re {
        ore: previous_resources.ore + state.robots.ore,
        clay: previous_resources.clay + state.robots.clay,
        obsidian: previous_resources.obsidian + state.robots.obsidian,
        geode: previous_resources.geode + state.robots.geode,
    };

    let mut types_of_buildable_robots_count = 0;
    if previous_resources.ore >= bp.ore as usize {
        types_of_buildable_robots_count += 1;

        let mut new_state = state.clone();
        new_state.resources = future_resources.clone();
        new_state.resources.ore -= bp.ore as usize;
        new_state.robots.ore += 1;

        next_steps.insert(new_state);
    }

    if previous_resources.ore >= bp.clay as usize {
        types_of_buildable_robots_count += 1;

        let mut new_state = state.clone();
        new_state.resources = future_resources.clone();
        new_state.resources.ore -= bp.clay as usize;
        new_state.robots.clay += 1;

        next_steps.insert(new_state);
    }

    if previous_resources.clay >= bp.obsidian.1 as usize
        && previous_resources.ore >= bp.obsidian.0 as usize
    {
        types_of_buildable_robots_count += 1;

        let mut new_state = state.clone();
        new_state.resources = future_resources.clone();
        new_state.resources.clay -= bp.obsidian.1 as usize;
        new_state.resources.ore -= bp.obsidian.0 as usize;
        new_state.robots.obsidian += 1;

        next_steps.insert(new_state);
    }

    if previous_resources.obsidian >= bp.geode.1 as usize
        && previous_resources.ore >= bp.geode.0 as usize
    {
        types_of_buildable_robots_count += 1;

        let mut new_state = state.clone();
        new_state.resources = future_resources.clone();
        new_state.resources.obsidian -= bp.geode.1 as usize;
        new_state.resources.ore -= bp.geode.0 as usize;
        new_state.robots.geode += 1;

        next_steps.insert(new_state);
    }

    if types_of_buildable_robots_count < 4 {
        next_steps.insert(State {
            resources: future_resources.clone(),
            robots: state.robots.clone(),
        });
    }
}

pub fn main() {
    let blueprints = get_blueprints();

    let first_state = State {
        resources: Re {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        robots: Ro {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
    };

    let mut results = vec![];

    for (j, bp) in blueprints.iter().enumerate() {
        let mut next_steps = HashSet::new();
        next_steps.insert(first_state.clone());

        for i in 1..=TIME {
            let mut new_next_steps: HashSet<State> = HashSet::new();
            for step in next_steps {
                adds_steps(&step, &bp, &mut new_next_steps);
            }

            let max_geode = new_next_steps
                .iter()
                .map(|s| s.resources.geode)
                .max()
                .unwrap();

            if max_geode >= 2 {
                new_next_steps = new_next_steps
                    .iter()
                    .filter(|s| s.resources.geode + 2 >= max_geode)
                    .map(|s| s.clone())
                    .collect::<HashSet<State>>();
            }

            println!(
                "{}:  steps: {},  geodes {}",
                i,
                new_next_steps.len(),
                max_geode
            );
            next_steps = new_next_steps;
        }

        let max_geode = next_steps.iter().map(|s| s.resources.geode).max().unwrap();
        println!("{}: Max geode: {}", j + 1, max_geode);
        results.push(max_geode)
    }

    let mut acc = 1;
    results.sort();
    results.reverse();
    results.iter().enumerate().for_each(|(i, r)| {
        if i <= 2 {
            acc *= r;
            println!("{} {}", r, acc)
        }
    });
}
