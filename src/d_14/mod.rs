pub mod input;

/*
const EXAMPLE: &str = "
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";
*/

fn get_paths() -> Vec<Vec<(usize, usize)>> {
    let lines = input::INPUT.lines().filter(|l| !l.is_empty());
    let paths = lines
        .map(|l| {
            l.split(" -> ")
                .map(|p| {
                    let mut parts = p.split(",");
                    let x = parts.next().unwrap().parse().unwrap();
                    let y = parts.next().unwrap().parse().unwrap();
                    (x, y)
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<Vec<(usize, usize)>>>();
    paths
}

fn get_rocks(paths: &Vec<Vec<(usize, usize)>>) -> Vec<(usize, usize)> {
    let mut rocks = Vec::new();

    for path in paths {
        for i in 0..path.len() - 1 {
            let start = path[i];
            let end = path[i + 1];
            rocks.push(start);
            rocks.push(end);

            if start.0 == end.0 {
                let min = start.1.min(end.1);
                let max = start.1.max(end.1);
                for y in min..max {
                    rocks.push((start.0, y));
                }
            } else {
                let min = start.0.min(end.0);
                let max = start.0.max(end.0);
                for x in min..max {
                    rocks.push((x, start.1));
                }
            }
        }
    }
    rocks
}

fn get_map(coordonates: &Vec<(usize, usize)>, fill: char) -> Vec<Vec<char>> {
    let mut map = vec![vec!['.'; 1000]; 200];
    let mut max_y = 0;

    coordonates.iter().for_each(|(x, y)| {
        if y > &max_y {
            max_y = *y;
        }
        map[*y][*x] = fill;
    });

    for i in 0..1000 {
        map[max_y + 2][i] = '~';
    }

    map
}

fn flow_sand_one(map: &mut Vec<Vec<char>>) -> bool {
    let mut sand = (500, 0);
    let mut stop = false;
    if map[0][500] == 'o' {
        return true;
    }
    loop {
        let (x, y) = sand;

        let next = (x, y + 1);

        match map.get(y + 1) {
            None => {
                stop = true;
                break;
            }
            Some(..) => {}
        }

        if map[next.1][next.0] == '.' {
            sand = next;
            continue;
        }

        let next = (x - 1, y + 1);
        if map[next.1][next.0] == '.' {
            sand = next;
            continue;
        }

        let next = (x + 1, y + 1);
        if map[next.1][next.0] == '.' {
            sand = next;
            continue;
        }

        map[y][x] = 'o';
        break;
    }
    stop
}

fn flow_sand(map: &mut Vec<Vec<char>>) -> usize {
    let mut sand = 0;
    loop {
        let stop = flow_sand_one(map);
        if stop {
            break;
        }
        sand += 1;
    }
    sand
}

pub fn main() {
    let paths = get_paths();
    let rocks = get_rocks(&paths);
    let mut map = get_map(&rocks, '#');
    let sand = flow_sand(&mut map);
    println!("Sand: {}", sand);
    map.iter().for_each(|line| {
        for c in line[490..510].iter() {
            print!("{}", c);
        }
        println!();
    });
}
