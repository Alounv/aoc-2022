use std::collections::HashSet;
use std::time::SystemTime;
pub mod input;

#[derive(Debug, Clone, Copy)]
enum Dir {
    N,
    S,
    W,
    E,
}

fn get_elves() -> HashSet<(isize, isize)> {
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
    let now = SystemTime::now();
    let mut elves = get_elves();
    //print_elves(&elves);

    'round: for round in 0..1100 {
        let mut new_elves = elves.clone();
        let dir = match round % 4 {
            0 => Dir::N,
            1 => Dir::S,
            2 => Dir::W,
            3 => Dir::E,
            _ => unreachable!(),
        };

        // PHASE 1
        let mut intentions = Vec::new();
        for (x, y) in &elves {
            let is_alone = get_is_alone(&elves, x, y);

            if !is_alone {
                let dirs = get_dirs(&dir);
                let (next_x, next_y) = get_next(&elves, dirs, x, y);
                let intention = (next_x, next_y, *x, *y);
                intentions.push(intention);
            }
        }

        // PHASE 2
        intentions.sort();
        for i in 0..intentions.len() {
            let current = intentions[i];

            if (current.0, current.1) == (current.2, current.3) {
                continue;
            }

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

            match elves.get(&(current.0, current.1)) {
                None => {}
                Some(pos) => {
                    println!("ALERT {} {}", pos.0, pos.1);
                    break 'round;
                }
            }

            new_elves.remove(&(current.2, current.3));
            new_elves.insert((current.0, current.1));
        }

        let time = now.elapsed().unwrap().as_secs();
        if round % 100 == 0 {
            println!("Round {}: time {}", round + 1, time);
        }

        if elves == new_elves {
            println!("Round {}: time {}", round + 1, time);
            break;
        }
        elves = new_elves;
    }
}
