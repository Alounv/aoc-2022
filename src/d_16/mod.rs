pub mod input;
use itertools::Itertools;
use std::collections::HashMap;

const EXAMPLE: &str = "
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

struct Valve {
    flow: u32,
    tunnels: Vec<String>,
}

fn get_valves(input: &str) -> HashMap<String, Valve> {
    let mut valves = HashMap::new();
    input.lines().filter(|l| !l.is_empty()).for_each(|l| {
        let parts = l.split_whitespace().collect::<Vec<_>>();
        let name = parts[1];
        let flow = parts[4].split('=').collect::<Vec<_>>()[1]
            .trim_end_matches(';')
            .parse::<u32>()
            .unwrap();
        let tunnels = parts[9..]
            .iter()
            .map(|t| t.trim_matches(',').to_string())
            .collect::<Vec<_>>();
        let valve = Valve { flow, tunnels };
        valves.insert(name.to_string(), valve);
    });
    valves
}

fn get_flow(valves: &HashMap<String, Valve>, open_valves: &Vec<String>) -> u32 {
    valves
        .iter()
        .filter(|(name, _v)| open_valves.contains(&name))
        .fold(0, |acc, (_name, v)| acc + v.flow)
}

fn get_move_actions(current_valve: &Valve, prev: &String) -> Vec<String> {
    let tunnels = current_valve.tunnels.clone();
    tunnels
        .iter()
        .filter(|t| t != &prev)
        .map(|t| t.clone())
        .collect::<Vec<String>>()
}

fn add_open_action(
    current_valve: &Valve,
    open_valves: &Vec<String>,
    loc: &String,
    paths: &mut Vec<String>,
) {
    let is_null = current_valve.flow == 0;
    let is_current_valve_open = open_valves.contains(&loc);
    if !is_null && !is_current_valve_open {
        paths.push(loc.clone())
    }
}

fn get_paths(
    valves: &HashMap<String, Valve>,
    loc: String,
    prev: String,
    open_valves: &Vec<String>,
    history: &Vec<String>,
) -> Vec<Vec<String>> {
    if open_valves.len() >= VALVES_WITH_FLOW {
        return vec![history.clone()];
    }

    let current_valve = valves.get(&loc).unwrap();
    let mut new_position = get_move_actions(current_valve, &prev);
    add_open_action(current_valve, open_valves, &loc, &mut new_position);

    let paths = new_position
        .iter()
        .map(|p| {
            let mut new_history = history.clone();
            new_history.push(p.clone());
            new_history
        })
        .collect::<Vec<Vec<String>>>();
    paths
}

fn get_open_valves(history: &Vec<String>) -> Vec<String> {
    let mut open_valves = vec![];
    for i in 1..history.len() {
        match history.get(i) {
            Some(loc) => {
                let prev = history.get(i - 1).unwrap();
                if prev == loc {
                    open_valves.push(loc.clone());
                }
            }
            None => {}
        }
    }
    open_valves.sort();
    open_valves
}

fn get_pressure_from_path(time: usize, path: &Vec<String>, valves: &HashMap<String, Valve>) -> u32 {
    let mut pressure = 0;
    let mut open_valves = vec![];
    for i in 1..=time {
        let diff = get_flow(valves, &open_valves);
        pressure += diff;

        match path.get(i) {
            Some(loc) => {
                let prev = path.get(i - 1).unwrap();
                if prev == loc {
                    open_valves.push(loc.clone());
                }
            }
            None => {}
        }
    }
    pressure
}

const TIME: usize = 26;
const VALVES_WITH_FLOW: usize = 15;

pub fn main() {
    let valves = get_valves(EXAMPLE);

    /* STAGE 1: Create paths */

    let start = "AA".to_string();
    let mut paths = vec![vec![start.clone()]];
    for i in 1..=TIME {
        let mut turn_paths = vec![];

        paths.iter().for_each(|h| {
            let l = h.last().unwrap();
            let mut p = "".to_string();
            if h.len() > 1 {
                p = h.get(h.len() - 2).unwrap().clone();
            }
            let open_valves = get_open_valves(h);
            let new_paths = get_paths(&valves, l.clone(), p.clone(), &open_valves, &h);
            turn_paths.extend(new_paths);
        });

        turn_paths.sort();
        turn_paths.dedup();
        paths = turn_paths;
        println!("{}, paths {}", i, paths.len());
    }
    println!("STAGE 1");

    /* STAGE 2: Add keys */

    let mut key_paths = HashMap::new();

    // keep best path for given open valves
    paths.iter().for_each(|p| {
        let pressure = get_pressure_from_path(TIME, p, &valves);
        let open_valves = get_open_valves(p);
        let key = open_valves.join("-"); // the key works because the valve or in alhabetical order
        let current_best = key_paths.get(&key);
        match current_best {
            Some((best_pressure, _)) => {
                if pressure > *best_pressure {
                    key_paths.insert(key, (pressure, p.clone()));
                }
            }
            None => {
                key_paths.insert(key, (pressure, p.clone()));
            }
        }
    });

    println!("STAGE 2: number of best paths {}", key_paths.len());

    /* STAGE 3: Create possible repartitions of valves between A and B */

    let mut valves_with_flow = valves
        .iter()
        .filter(|(_name, v)| v.flow > 0)
        .map(|(name, _v)| name.clone())
        .collect::<Vec<String>>();

    valves_with_flow.sort();

    let count = valves_with_flow.len();

    let mut combinations_for_one = vec![];
    for i in 1..=count {
        let combi = valves_with_flow
            .iter()
            .map(|v| v.clone())
            .combinations(i)
            .collect::<Vec<Vec<String>>>();
        combinations_for_one.extend(combi);
    }

    let mut combinations = vec![];

    combinations_for_one.iter().for_each(|a| {
        combinations_for_one
            .iter()
            .filter(|b| {
                // keep only combination without duplicates
                !a.iter().any(|v| b.contains(v))
            })
            .for_each(|b| {
                let mut a = a.clone();
                let mut b = b.clone();
                a.sort();
                b.sort();
                combinations.push((a, b));
            });
    });

    println!("STAGE 3: {} combinaisons", combinations.len());

    /* STAGE 4: Associate the pressure to each combination */

    let mut best = (0, vec![], vec![]);
    combinations.iter().for_each(|(a, b)| {
        let a_key = a.join("-");
        let b_key = b.join("-");
        let empty = (0, vec![]);
        let (a_pressure, a_path) = key_paths.get(&a_key).unwrap_or(&empty);
        let (b_pressure, b_path) = key_paths.get(&b_key).unwrap_or(&empty);
        let pressure = a_pressure + b_pressure;

        if pressure > best.0 {
            best = (pressure.clone(), a_path.clone(), b_path.clone());
        }
    });

    println!("STAGE 4: best combination {:?}", best);
}
