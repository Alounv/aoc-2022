pub mod input;

/*
const EXAMPLE: &str = "
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";
*/

const CHARS: [(char, usize); 28] = [
    ('a', 1),
    ('b', 2),
    ('c', 3),
    ('d', 4),
    ('e', 5),
    ('f', 6),
    ('g', 7),
    ('h', 8),
    ('i', 9),
    ('j', 10),
    ('k', 11),
    ('l', 12),
    ('m', 13),
    ('n', 14),
    ('o', 15),
    ('p', 16),
    ('q', 17),
    ('r', 18),
    ('s', 19),
    ('t', 20),
    ('u', 21),
    ('v', 22),
    ('w', 23),
    ('x', 24),
    ('y', 25),
    ('z', 26),
    ('S', 1),
    ('E', 26),
];

fn get_map() -> Vec<Vec<char>> {
    return input::INPUT
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();
}

fn get_choices(current: (usize, usize), map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let max_x = map[0].len() as usize;
    let max_y = map.len() as usize;
    let x = current.0;
    let y = current.1;

    let mut c: Vec<(usize, usize)> = Vec::new();
    if x > 0 {
        c.push((x - 1, y))
    }
    if y > 0 {
        c.push((x, y - 1))
    }
    if x < max_x - 1 {
        c.push((x + 1, y))
    }
    if y < max_y - 1 {
        c.push((x, y + 1))
    }
    c
}

fn get_char_height(c: char) -> usize {
    for (char, height) in CHARS.iter() {
        if *char == c {
            return *height;
        }
    }
    return 0;
}

fn get_height((x, y): (usize, usize), map: &Vec<Vec<char>>) -> usize {
    return get_char_height(map[y][x]);
}

fn is_move_possible(current: (usize, usize), next: (usize, usize), map: &Vec<Vec<char>>) -> bool {
    let current_height = get_height(current, map);
    let next_height = get_height(next, map);
    return current_height <= next_height + 1;
}

fn is_move_explored(point: (usize, usize), explored: &Vec<(usize, usize)>) -> bool {
    explored.iter().any(|p| *p == point)
}

fn get_possible_choices(
    current: (usize, usize),
    map: &Vec<Vec<char>>,
    explored: &Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let choices = get_choices(current, map);
    let choices = choices
        .iter()
        .filter(|&&c| is_move_possible(current, c, map) && !is_move_explored(c, &explored))
        .map(|&c| c)
        .collect::<Vec<(usize, usize)>>();
    choices
}

fn explore_choices(
    paths: &Vec<Vec<(usize, usize)>>,
    index: usize,
    explored: &mut Vec<(usize, usize)>,
    possible_choices: Vec<(usize, usize)>,
) -> Vec<Vec<(usize, usize)>> {
    let mut new_paths: Vec<Vec<(usize, usize)>> = paths.clone();

    for (i, choice) in possible_choices.iter().enumerate() {
        explored.push(*choice);
        if i == 0 {
            new_paths[index].push(*choice);
        } else {
            let mut new_path = paths[index].clone();
            new_path.push(*choice);
            new_paths.push(new_path);
        }
    }

    return new_paths;
}

fn explore_one_step(
    paths: &Vec<Vec<(usize, usize)>>,
    success_paths: &mut Vec<Vec<(usize, usize)>>,
    explored: &mut Vec<(usize, usize)>,
    map: &Vec<Vec<char>>,
) -> Vec<Vec<(usize, usize)>> {
    let mut new_paths = paths.clone();
    let mut paths_to_kill: Vec<usize> = Vec::new();
    let steps_count = success_paths
        .iter()
        .map(|p| p.len() - 1)
        .min()
        .unwrap_or(1000);

    for (index, path) in paths.iter().enumerate() {
        let position = path[path.len() - 1];

        if path.len() - 1 > steps_count {
            paths_to_kill.push(index);
            continue;
        }

        let height = get_height(position, map);
        if height == 1 {
            success_paths.push(path.clone());
            paths_to_kill.push(index);
            continue;
        }

        let possible_choices = get_possible_choices(position, map, explored);

        if possible_choices.len() == 0 {
            paths_to_kill.push(index);
            continue;
        }

        new_paths = explore_choices(&new_paths, index, explored, possible_choices);
    }

    paths_to_kill.reverse();

    paths_to_kill.iter().for_each(|i| {
        new_paths.remove(*i);
    });

    new_paths
}

fn explore(
    paths: &Vec<Vec<(usize, usize)>>,
    map: &Vec<Vec<char>>,
    success_paths: &mut Vec<Vec<(usize, usize)>>,
) {
    let mut explored = vec![(0, 0)];
    let mut new_paths = paths.clone();

    while new_paths.len() != 0 {
        new_paths = explore_one_step(&new_paths, success_paths, &mut explored, map);
    }
}

pub fn main() {
    let map = get_map();
    let start = (146, 20);
    //let end = (5, 2);

    let mut success_paths: Vec<Vec<(usize, usize)>> = Vec::new();

    let paths = vec![vec![start]];
    explore(&paths, &map, &mut success_paths);
    let global_min = success_paths.iter().map(|p| p.len() - 1).min().unwrap_or(0);
    if global_min > 0 {
        println!("steps: {}", global_min);
    }
}
