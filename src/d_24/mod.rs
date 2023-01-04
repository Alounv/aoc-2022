pub mod input;
use colored::Colorize;
use std::collections::{HashMap, HashSet};
use std::io::stdin;

const EXAMPLE: &str = "
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

fn get_initial_map() -> Vec<Vec<char>> {
    let mut blizzard = Vec::new();
    let input = input::I;
    //let input = EXAMPLE;
    for line in input.lines().filter(|l| !l.is_empty()) {
        blizzard.push(line.chars().collect());
    }
    blizzard
}

fn get_next_blizzard_position(
    char: &char,
    x: usize,
    y: usize,
    map: &Vec<Vec<char>>,
) -> (usize, usize) {
    let row_len = map[0].len();
    let col_len = map.len();

    let (mut next_x, mut next_y) = match char {
        '>' => (x + 1, y),
        '<' => (x - 1, y),
        '^' => (x, y - 1),
        'v' => (x, y + 1),
        _ => panic!("Unknown char: {}", char),
    };

    if map[next_y][next_x] == '#' {
        if next_x == row_len - 1 {
            next_x = 1;
        } else if next_x == 0 {
            next_x = row_len - 2;
        } else if next_y == col_len - 1 {
            next_y = 1;
        } else if next_y == 0 {
            next_y = col_len - 2;
        }
    }
    (next_x, next_y)
}

fn update_blizzards(
    blizzards: &mut HashMap<(usize, usize), Vec<char>>,
    char: &char,
    x: usize,
    y: usize,
) {
    blizzards
        .entry((x, y))
        .and_modify(|v| v.push(*char))
        .or_insert(vec![*char]);
}

fn get_initial_blizzards(map: &Vec<Vec<char>>) -> HashMap<(usize, usize), Vec<char>> {
    let mut next_blizzards: HashMap<(usize, usize), Vec<char>> = HashMap::new();
    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, char)| {
            if *char != '#' && *char != '.' {
                update_blizzards(&mut next_blizzards, char, x, y);
            }
        })
    });
    next_blizzards
}

fn get_next_blizzards(
    blizzards: &HashMap<(usize, usize), Vec<char>>,
    initial_map: &Vec<Vec<char>>,
) -> HashMap<(usize, usize), Vec<char>> {
    let mut next_blizzards: HashMap<(usize, usize), Vec<char>> = HashMap::new();
    blizzards.iter().for_each(|((x, y), chars)| {
        chars.iter().for_each(|char| {
            let (next_x, next_y) = get_next_blizzard_position(char, *x, *y, initial_map);
            update_blizzards(&mut next_blizzards, char, next_x, next_y);
        })
    });
    next_blizzards
}

fn get_next_map(
    initial_map: &Vec<Vec<char>>,
    blizzards: &HashMap<(usize, usize), Vec<char>>,
) -> Vec<Vec<char>> {
    let row_len = initial_map[0].len();
    let col_len = initial_map.len();
    let mut next_map = initial_map.clone();

    for y in 1..col_len - 1 {
        for x in 1..row_len - 1 {
            let char: char = match blizzards.get(&(x, y)) {
                Some(v) => {
                    if v.len() > 1 {
                        v.len().to_string().chars().next().unwrap()
                    } else {
                        v[0]
                    }
                }
                None => '.',
            };
            next_map[y][x] = char;
        }
    }

    next_map
}

const NEIGHBOURS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn get_water(
    map: &Vec<Vec<char>>,
    water: &HashSet<(usize, usize)>,
    start: &(usize, usize),
) -> HashSet<(usize, usize)> {
    let mut next_water = HashSet::new();
    next_water.insert(*start);
    water.iter().for_each(|(x, y)| {
        for (dx, dy) in NEIGHBOURS.iter() {
            let next_x = *x as isize + dx;
            let next_y = *y as isize + dy;
            if next_x < 0
                || next_y < 0
                || next_x >= map[0].len() as isize
                || next_y >= map.len() as isize
            {
                continue;
            }
            let next_x = next_x as usize;
            let next_y = next_y as usize;
            if map[*y][*x] == '.' {
                next_water.insert((*x, *y));
            }
            if map[next_y][next_x] == '.' {
                next_water.insert((next_x, next_y));
            }
        }
    });
    next_water
}

fn print_map(map: &Vec<Vec<char>>, water: &HashSet<(usize, usize)>) {
    for (y, row) in map.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            let colored = match char {
                '#' => format!("{}", char).black(),
                '.' => {
                    if water.contains(&(x, y)) {
                        format!("{}", '~').on_blue()
                    } else {
                        format!("{}", char).black()
                    }
                }
                _ => format!("{}", char).red(),
            };
            print!("{}", colored);
        }
        println!();
    }
    println!();
}

fn clean_previous_print(map: &Vec<Vec<char>>) {
    for _ in 0..=map.len() + 1 {
        print!("\x1B[1A\x1B[2K");
    }
}

const START: (usize, usize) = (1, 0);

pub fn main() {
    let mut map = get_initial_map();
    let mut end = (map[0].len() - 2, map.len() - 1);
    let mut start = START;
    let mut blizzards = get_initial_blizzards(&map);
    let mut water = HashSet::new();
    let mut phase = 0;
    water.insert(start);

    print_map(&map, &water);

    for i in 1..400 {
        clean_previous_print(&map);
        //stdin().read_line(&mut String::new()).unwrap();

        blizzards = get_next_blizzards(&blizzards, &map);
        map = get_next_map(&map, &blizzards);
        water = get_water(&map, &water, &start);

        println!("After minute {}", i);
        print_map(&map, &water);

        if water.contains(&end) {
            let new_end = start;
            let new_start = end;
            phase += 1;
            water = HashSet::new();
            water.insert(new_start);
            end = new_end;
            start = new_start;

            println!("Phase {} {:?} {:?}", phase, end, start);

            if phase == 3 {
                break;
            }
        }
    }
}
