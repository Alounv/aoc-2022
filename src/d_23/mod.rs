use std::collections::HashSet;
use std::io;
pub mod input;

const EXAMPLE: &str = "
..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............
";

#[derive(Debug, Clone, Copy)]
enum Dir {
    N,
    S,
    W,
    E,
}

const ROUNDS: usize = 10;

fn get_elves() -> HashSet<(isize, isize)> {
    //let input = EXAMPLE;
    let input = input::INPUT;
    let mut elves = HashSet::new();
    for (y, line) in input.lines().filter(|l| !l.is_empty()).enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert((x as isize, y as isize));
            }
        }
    }
    elves
}

fn print_elves(elves: &HashSet<(isize, isize)>) {
    let min_x = elves.iter().map(|(x, _)| x).min().unwrap();
    let min_y = elves.iter().map(|(_, y)| y).min().unwrap();
    let max_x = elves.iter().map(|(x, _)| x).max().unwrap();
    let max_y = elves.iter().map(|(_, y)| y).max().unwrap();

    let mut grid = vec![vec!['.'; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    let regions_count = (max_x - min_x + 1) * (max_y - min_y + 1);

    for elf in elves {
        grid[(elf.1 - min_y) as usize][(elf.0 - min_x) as usize] = '#';
    }
    for line in grid {
        println!("{}", line.iter().collect::<String>());
    }
    println!("{} empty regions", regions_count - elves.len() as isize);
}

fn get_dirs(dir: &Dir) -> [Dir; 4] {
    let dirs = match dir {
        Dir::N => [Dir::N, Dir::S, Dir::W, Dir::E],
        Dir::S => [Dir::S, Dir::W, Dir::E, Dir::N],
        Dir::W => [Dir::W, Dir::E, Dir::N, Dir::S],
        Dir::E => [Dir::E, Dir::N, Dir::S, Dir::W],
    };
    dirs
}

fn get_next(
    elves: &HashSet<(isize, isize)>,
    dirs: [Dir; 4],
    x: &isize,
    y: &isize,
) -> (isize, isize) {
    let mut intention = (*x, *y);
    for d in dirs {
        match d {
            Dir::N => {
                if elves.contains(&(*x, y - 1))
                    || elves.contains(&(x - 1, y - 1))
                    || elves.contains(&(x + 1, y - 1))
                {
                    continue;
                }
                intention.1 -= 1;
                break;
            }
            Dir::S => {
                if elves.contains(&(*x, y + 1))
                    || elves.contains(&(x - 1, y + 1))
                    || elves.contains(&(x + 1, y + 1))
                {
                    continue;
                }
                intention.1 += 1;
                break;
            }
            Dir::W => {
                if elves.contains(&(x - 1, *y))
                    || elves.contains(&(x - 1, y - 1))
                    || elves.contains(&(x - 1, y + 1))
                {
                    continue;
                }
                intention.0 -= 1;
                break;
            }
            Dir::E => {
                if elves.contains(&(x + 1, *y))
                    || elves.contains(&(x + 1, y - 1))
                    || elves.contains(&(x + 1, y + 1))
                {
                    continue;
                }
                intention.0 += 1;
                break;
            }
        }
    }
    intention
}

fn get_is_alone(elves: &HashSet<(isize, isize)>, x: &isize, y: &isize) -> bool {
    let mut is_alone = true;
    for elf in elves {
        if elf.0 == *x && elf.1 == *y {
            continue;
        }
        if (elf.0 - x).abs() <= 1 && (elf.1 - y).abs() <= 1 {
            is_alone = false;
            break;
        }
    }
    is_alone
}

pub fn main() {
    let mut elves = get_elves();
    print_elves(&elves);

    for round in 0..ROUNDS {
        let dir = match round % 4 {
            0 => Dir::N,
            1 => Dir::S,
            2 => Dir::W,
            3 => Dir::E,
            _ => unreachable!(),
        };

        // PHASE 0
        let mut buffer = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer);

        // PHASE 1
        let mut intentions = Vec::new();
        for (x, y) in &elves {
            let mut intention = (*x, *y, *x, *y);
            let is_alone = get_is_alone(&elves, x, y);

            if !is_alone {
                let dirs = get_dirs(&dir);
                let (next_x, next_y) = get_next(&elves, dirs, x, y);
                intention = (next_x, next_y, *x, *y);
            }

            intentions.push(intention);
        }

        // PHASE 2
        intentions.sort();
        for i in 0..intentions.len() {
            let current = intentions[i];

            if i > 1 {
                let p = intentions.get(i - 1).unwrap();
                if p.0 == current.0 && p.1 == current.1 {
                    continue;
                }
            }
            if i < intentions.len() - 1 {
                let n = intentions.get(i + 1).unwrap();
                if n.0 == current.0 && n.1 == current.1 {
                    continue;
                }
            }

            elves.remove(&(current.2, current.3));
            elves.insert((current.0, current.1));
        }

        println!("Round {}", round + 1);
        print_elves(&elves);
    }
}
