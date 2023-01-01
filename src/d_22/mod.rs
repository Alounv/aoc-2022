pub mod input;

/*
const EXAMPLE: &str = "
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

const TEST: &str = "
    ........
    ........
    ........
    ........
    ....
    ....
    ....
    ....
........
........
........
........
....
....
....
....

5-5-5-5-5-5-5-5-5-5-5-5-5-5-5-5-5-5-5-5
";
*/

const S: usize = 50;

struct Interface {
    new_dir: u8,
    get_x: fn(usize, usize) -> usize,
    get_y: fn(usize, usize) -> usize,
}

fn get_interface(key: &str) -> Interface {
    match key {
        // S/0 - 2S/0       T-O dir = 0, x = 0,             y = x - S + 3S
        // 4S/0 - 3S/0      O-T dir = 1, x = y - 3S + S,    y = 0
        "T-O" => Interface {
            new_dir: 0,
            get_x: |_, _| 0,
            get_y: |x, _| x + 2 * S,
        },
        "O-T" => Interface {
            new_dir: 1,
            get_x: |_, y| y - 2 * S,
            get_y: |_, _| 0,
        },
        // 2S/0 - 3S/0      R-O dir = 3, x = x - 2S,        y = 4S - 1
        // 4S/2S - 4S/0     O-R dir = 1, x = x + 2S,        y = 0
        "R-O" => Interface {
            new_dir: 3,
            get_x: |x, _| x - 2 * S,
            get_y: |_, _| 4 * S - 1,
        },
        "O-R" => Interface {
            new_dir: 1,
            get_x: |x, _| x + 2 * S,
            get_y: |_, _| 0,
        },
        // 3S/0 - 3S/S      R-B dir = 2, x = 2S - 1,        y = 2S + (S - y)
        // 2S/2S - 3S/2S    B-R dir = 2, x = 3S - 1,        y = 3S - y
        "R-B" => Interface {
            new_dir: 2,
            get_x: |_, _| 2 * S - 1,
            get_y: |_, y| 2 * S + (S - y) - 1,
        },
        "B-R" => Interface {
            new_dir: 2,
            get_x: |_, _| 3 * S - 1,
            get_y: |_, y| 3 * S - y - 1,
        },
        // 3S/S - 2S/S      R-F dir = 2, x = 2S - 1,        y = S + x - 2S
        // 2S/S - 2S/2S     F-R dir = 3, x = 2S + y - S,    y = S
        "R-F" => Interface {
            new_dir: 2,
            get_x: |_, _| 2 * S - 1,
            get_y: |x, _| S + x - 2 * S,
        },
        "F-R" => Interface {
            new_dir: 3,
            get_x: |_, y| 2 * S + y - S,
            get_y: |_, _| S - 1,
        },
        // 3S/S - 3S/2S     B-O dir = 2, x = S - 1,         y = 3S + x - S
        // 3S/2S - 4S/2S    O-B dir = 3, x = y - 3S + S,    y = 3S - 1
        "B-O" => Interface {
            new_dir: 2,
            get_x: |_, _| S - 1,
            get_y: |x, _| 3 * S + x - S,
        },
        "O-B" => Interface {
            new_dir: 3,
            get_x: |_, y| y - 3 * S + S,
            get_y: |_, _| 3 * S - 1,
        },
        // 3S/0 - 2S/0      L-T dir = 0, x = S - 1,         y = S - (y - 2S)
        // S/S - S/0        T-L dir = 0, x = 0,             y = 2S + (S - x)
        "L-T" => Interface {
            new_dir: 0,
            get_x: |_, _| S,
            get_y: |_, y| S - (y - 2 * S) - 1,
        },
        "T-L" => Interface {
            new_dir: 0,
            get_x: |_, _| 0,
            get_y: |_, y| 2 * S + (S - y) - 1,
        },
        // 2S/0 - 2S/S      L-F dir = 0, x = S - 1,         y = S + x
        // 2S/S - S/S       F-L dir = 1, x = y - S,         y = S - 1
        "L-F" => Interface {
            new_dir: 0,
            get_x: |_, _| S,
            get_y: |x, _| S + x,
        },
        "F-L" => Interface {
            new_dir: 1,
            get_x: |_, y| y - S,
            get_y: |_, _| 2 * S,
        },
        _ => panic!("Unknown key: {}", key),
    }
}

fn get_input() -> (Vec<Vec<char>>, Vec<(usize, char)>) {
    let input = input::INPUT;
    //let input = TEST;
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let grid = parts[0]
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let raw_instructions = parts[1]
        .lines()
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();

    let mut instructions = vec![];
    let mut steps_count = "".to_string();
    for c in raw_instructions.iter() {
        match c.to_digit(10) {
            Some(_) => steps_count.push(*c),
            None => {
                instructions.push((steps_count.parse::<usize>().unwrap(), *c));
                steps_count = "".to_string();
            }
        }
    }
    instructions.push((steps_count.parse::<usize>().unwrap(), ' '));
    (grid, instructions)
}

const START: (usize, usize, u8) = (50, 0, 0);

pub fn main() {
    let (mut grid, instructions) = get_input();

    let (x, y, dir) = START;
    let mut position = (x, y, dir);

    grid[y][x] = '>';

    for i in 0..instructions.len() {
        let (steps, turn) = instructions[i];
        /*
        println!("{:?}", position);
        for row in grid.iter() {
            println!("{:?}", row.iter().collect::<String>());
        }
        println!("{:?} {:?}", steps, turn);
        let mut buffer = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer);
        */
        let mut next = position;

        // move
        for _ in 0..steps {
            let (x, y, dir) = next;
            let mut next_x = x as isize;
            let mut next_y = y as isize;
            let mut next_dir = dir;

            match dir {
                0 => next_x += 1,
                1 => next_y += 1,
                2 => next_x -= 1,
                3 => next_y -= 1,
                _ => panic!("invalid direction"),
            };

            let mut key: &str = "";
            match dir {
                0 => {
                    if next_x as usize >= grid[y].len() {
                        if y < S {
                            key = "R-B";
                        } else if y < 2 * S {
                            key = "F-R";
                        } else if y < 3 * S {
                            key = "B-R";
                        } else {
                            key = "O-B";
                        }
                    }
                }
                1 => {
                    if next_y as usize >= grid.len()
                        || grid[next_y as usize].get(x).unwrap_or(&' ') == &' '
                    {
                        if x < S {
                            key = "O-R";
                        } else if x < 2 * S {
                            key = "B-O";
                        } else {
                            key = "R-F";
                        }
                    }
                }
                2 => {
                    if next_x < 0 || grid[y][next_x as usize] == ' ' {
                        if y < S {
                            key = "T-L";
                        } else if y < 2 * S {
                            key = "F-L";
                        } else if y < 3 * S {
                            key = "L-T";
                        } else {
                            key = "O-T";
                        }
                    }
                }
                3 => {
                    if next_y < 0 || grid[next_y as usize][x] == ' ' {
                        if x < S {
                            key = "L-F";
                        } else if x < 2 * S {
                            key = "T-O";
                        } else {
                            key = "R-O";
                        }
                    }
                }
                _ => panic!("invalid direction"),
            }

            if !key.is_empty() {
                //    println!("key: {}", key);
                //   println!("next: {} {} {}", next_x, next_y, next_dir);
                let interface = get_interface(key);
                next_x = (interface.get_x)(x as usize, y as usize) as isize;
                next_y = (interface.get_y)(x as usize, y as usize) as isize;
                next_dir = interface.new_dir;
                //  println!("next: {} {} {}", next_x, next_y, next_dir);
            }

            // break if we hit a wall
            if grid[next_y as usize][next_x as usize] == '#' {
                next = (x, y, dir); // stay in place
                break;
            }

            next = (next_x as usize, next_y as usize, next_dir);
            grid[next_y as usize][next_x as usize] = match next_dir {
                0 => '>',
                1 => 'v',
                2 => '<',
                3 => '^',
                _ => panic!("invalid direction"),
            };
        }

        // turn
        next.2 = match turn {
            'R' => (next.2 + 1) % 4,
            'L' => (next.2 + 3) % 4,
            _ => next.2,
        };

        grid[next.1][next.0] = match next.2 {
            0 => '>',
            1 => 'v',
            2 => '<',
            3 => '^',
            _ => panic!("invalid direction"),
        };

        position = next;
    }
    for row in grid.iter() {
        println!("{:?}", row.iter().collect::<String>());
    }
    println!("{:?}", position);
    let (x, y, d) = position;
    println!("{:?}", (y + 1) * 1_000 + (x + 1) * 4 + d as usize);
}
