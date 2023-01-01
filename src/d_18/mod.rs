use std::collections::HashMap;

pub mod input;

/*
const EXAMPLE: &str = "
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";
*/

fn get_cubes() -> Vec<Vec<isize>> {
    let input = input::INPUT;
    //let input = EXAMPLE;

    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split(',')
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<Vec<isize>>>()
}

fn get_extremes(cubes: &Vec<Vec<isize>>) -> Vec<(isize, isize)> {
    let mut extremes = vec![(0, 0), (0, 0), (0, 0)];
    for cube in cubes {
        for i in 0..3 {
            let c = cube[i];
            let current_extreme = extremes[i];
            if c < current_extreme.0 {
                extremes[i].0 = cube[i];
            }
            if c > current_extreme.1 {
                extremes[i].1 = cube[i];
            }
        }
    }
    return extremes.iter().map(|e| (e.0 - 1, e.1 + 1)).collect();
}

fn get_neighbourgs(c: &Vec<isize>) -> Vec<Vec<isize>> {
    vec![
        vec![c[0] - 1, c[1], c[2]],
        vec![c[0] + 1, c[1], c[2]],
        vec![c[0], c[1] - 1, c[2]],
        vec![c[0], c[1] + 1, c[2]],
        vec![c[0], c[1], c[2] - 1],
        vec![c[0], c[1], c[2] + 1],
    ]
}

fn faces_touching_water(c: &Vec<isize>, water: &HashMap<Vec<isize>, bool>) -> u8 {
    let neighbours = get_neighbourgs(c);
    let mut touch_faces_count = 0;
    for neighbour in neighbours {
        if water.contains_key(&neighbour) {
            touch_faces_count += 1;
        }
    }
    return touch_faces_count;
}

fn get_is_touching_water(cube: &Vec<isize>, water: &HashMap<Vec<isize>, bool>) -> bool {
    return faces_touching_water(cube, water) > 0;
}

pub fn main() {
    let cubes = get_cubes();
    let rocks = cubes.len();

    let extremes = get_extremes(&cubes);

    let ex = extremes[0];
    let ey = extremes[1];
    let ez = extremes[2];

    let mut water: HashMap<Vec<isize>, bool> = HashMap::new();

    let space = ((ex.1 - ex.0) * (ey.1 - ey.0) * (ez.1 - ez.0)) as usize;
    println!("spaces: {}", space);
    println!("rocks: {}", rocks);
    let start = vec![ex.0, ey.0, ez.0];
    water.insert(start.clone(), true);

    let mut cubes_to_explore = vec![start.clone()];

    while cubes_to_explore.len() > 0 {
        let mut new_cubes_to_explore = vec![];
        for c in cubes_to_explore.iter() {
            let is_rock = cubes.contains(&c);

            if !is_rock {
                let is_water = get_is_touching_water(&c, &water);
                if is_water {
                    water.insert(c.clone(), true);
                }
            }

            let neighbours = get_neighbourgs(&c.clone());

            let possible_explorations = neighbours
                .iter()
                .filter(|n| {
                    if n[0] > ex.1
                        || n[1] > ey.1
                        || n[2] > ez.1
                        || n[0] < ex.0
                        || n[1] < ey.0
                        || n[2] < ez.0
                    {
                        // do not consider out of bound neighbours
                        return false;
                    }

                    if cubes.contains(n) {
                        // do not consider neighbours that are rocks
                        return false;
                    }

                    if water.contains_key(*n) {
                        // do not consider neighbours that are already water
                        return false;
                    }
                    true
                })
                .map(|n| n.clone())
                .collect::<Vec<Vec<isize>>>();

            new_cubes_to_explore.extend(possible_explorations);
        }
        new_cubes_to_explore.sort();
        new_cubes_to_explore.dedup();
        cubes_to_explore = new_cubes_to_explore;
    }

    println!("Water {}", water.len());

    let mut surface: usize = 0;
    let mut wet_rocks = 0;
    for i in 0..rocks {
        let faces_touching_water = faces_touching_water(&cubes[i], &water);
        surface += faces_touching_water as usize;
        if faces_touching_water > 0 {
            wet_rocks += 1;
        }
    }

    println!("Wet rocks {}", wet_rocks);

    println!("Surface {}", surface);
    assert_eq!(surface, 2012);
}
