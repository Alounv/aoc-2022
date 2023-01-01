//use std::collections::HashMap;

pub mod input;

const BLOCK_COUNT: usize = 2040 + 1725 * 579710143;
//const BLOCK_COUNT: usize = 2040 + 1725 * 1;
//const BLOCK_COUNT: usize = 2040 + 1725 * 1 + 1285;
//const BLOCK_COUNT: usize = 49 + 35 * 10;

/*
const W: usize = 7;

const EXAMPLE: &str = "
>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
";

#[derive(Debug, Clone)]
struct Block {
    shape: Vec<(usize, usize)>,
    left: isize,
    bottom: isize,
}

#[derive(Debug, Clone)]
enum Move {
    Lateral(Lateral),
    Down,
}

#[derive(Debug, Clone)]
enum Lateral {
    Left,
    Right,
}

const INTIAL_LEFT: usize = 2;
const INITIAL_BOTTOM: usize = 3;

// use left-bottom limit as origin
const H: [(usize, usize); 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];
const X: [(usize, usize); 5] = [(1, 1), (1, 0), (0, 1), (2, 1), (1, 2)];
const L: [(usize, usize); 5] = [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];
const I: [(usize, usize); 4] = [(0, 0), (0, 1), (0, 2), (0, 3)];
const O: [(usize, usize); 4] = [(0, 0), (1, 0), (0, 1), (1, 1)];

fn get_jet_sequence() -> Vec<Lateral> {
    let mut seq = Vec::new();
    for c in input::INPUT.chars() {
        match c {
            '>' => seq.push(Lateral::Right),
            '<' => seq.push(Lateral::Left),
            _ => (),
        }
    }
    seq
}

fn get_block(index: usize, highest_point: usize) -> Block {
    Block {
        shape: match index % 5 {
            1 => H.to_vec(),
            2 => X.to_vec(),
            3 => L.to_vec(),
            4 => I.to_vec(),
            0 => O.to_vec(),
            _ => panic!("invalid index"),
        },
        left: INTIAL_LEFT as isize,
        bottom: (highest_point + INITIAL_BOTTOM) as isize,
    }
}

fn update_jet_index(jet_index: &mut usize, jet_sequence: &Vec<Lateral>) {
    let next_index = jet_index.clone() + 1;
    match jet_sequence.get(next_index) {
        Some(_) => *jet_index += 1,
        None => *jet_index = 0,
    }
}

fn is_move_valid(new_block: &Block, area: &Vec<[char; W]>) -> bool {
    let is_valid = new_block.shape.iter().all(|(x, y)| {
        let x = new_block.left + *x as isize;
        let y = new_block.bottom + *y as isize;
        if x < 0 || x > 6 || y < 0 {
            return false;
        }
        match area.get(y as usize) {
            Some(row) => row[x as usize] != '#',
            None => true,
        }
    });
    is_valid
}

fn move_block(block: &mut Block, area: &Vec<[char; W]>, move_direction: Move) -> bool {
    let mut new_block = block.clone();

    match move_direction {
        Move::Lateral(direction) => match direction {
            Lateral::Left => new_block.left -= 1,
            Lateral::Right => new_block.left += 1,
        },
        Move::Down => new_block.bottom -= 1,
    }

    let is_valid = is_move_valid(&new_block, area);
    if is_valid {
        *block = new_block;
    }
    is_valid
}

fn update_area(block: &Block, area: &mut Vec<[char; W]>, highest_point: usize) -> usize {
    let mut new_highest_point = highest_point;
    for (x, y) in &block.shape {
        let x = block.left as usize + x;
        let y = block.bottom as usize + y;

        if y + 1 > new_highest_point {
            new_highest_point = y + 1;
        }

        match area.get_mut(y) {
            Some(row) => row[x] = '#',
            None => {
                area.push([' '; W]);
                area.push([' '; W]);
                area.push([' '; W]);
                area[y][x] = '#';
            }
        }
    }
    new_highest_point
}

fn record_jet_index(jet_index: usize, highest_point: usize, jet_record: &mut Vec<(usize, usize)>) {
    jet_record.push((jet_index, highest_point));
}

fn are_vectors_equal_so_far(a: &Vec<usize>, b: &Vec<usize>) -> bool {
    let count = a.len().min(b.len());
    for i in 0..count {
        if a[i] != b[i] {
            return false;
        }
    }
    true
}

fn identify_pattern(jet_record: &Vec<(usize, usize)>) -> (Vec<usize>, usize, usize, usize) {
    let mut total_pattern = vec![];
    let mut pattern_pairs = HashMap::<String, (Vec<usize>, Vec<usize>)>::new();

    for (jet_index, highest_point) in jet_record.iter() {
        // create new pattern_pairs if there is a duplicate
        let duplicate_indexes = total_pattern
            .iter()
            .enumerate()
            .filter(|(_, value)| **value == *jet_index)
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        duplicate_indexes.iter().for_each(|start_index| {
            // possible pattern, register reference and start comparing
            pattern_pairs.insert(
                format!("{}-{}", *highest_point, jet_index),
                (total_pattern[*start_index..].to_vec(), vec![]),
            );
        });

        // for each pattern_pairs, update then compare both
        if pattern_pairs.len() > 0 {
            let keys = pattern_pairs.keys().cloned().collect::<Vec<String>>();
            for key in keys {
                let (r, c) = pattern_pairs.get_mut(&key).unwrap();
                c.push(*jet_index);
                let same_size = r.len() == c.len();

                if !are_vectors_equal_so_far(r, c) {
                    pattern_pairs.remove(&key);
                } else if same_size {
                    // we have a pattern
                    let pattern_start =
                        key.split("-").next().unwrap().parse::<usize>().unwrap() - 1;
                    let blocks_before = total_pattern.len() - r.len();
                    return (r.clone(), pattern_start, *highest_point, blocks_before);
                }
            }
        }

        total_pattern.push(*jet_index);
    }
    (vec![], 0, 0, 0)
}

fn print_area(area: &Vec<[char; W]>, jet_record: &Vec<(usize, usize)>) {
    println!(" ");
    area.iter().enumerate().rev().for_each(|(i, row)| {
        let row: String = row.iter().collect();
        let jet = jet_record
            .iter()
            .find(|(_, highest_point)| *highest_point == i)
            .map(|(jet, _)| jet.to_string())
            .unwrap_or(" ".to_string());
        println!("| {} | {} - {}", row, i, jet);
    });
    println!("*---------*");
}
*/

pub fn main() {
    /*
    let mut area: Vec<[char; W]> = vec![[' '; W]; W];
    let mut highest_point: usize = 0;
    let jet_sequence = get_jet_sequence();
    let mut jet_index = 0;
    let mut jet_record: Vec<(usize, usize)> = Vec::new();

    for block_index in 1..=BLOCK_COUNT {
        let mut block = get_block(block_index, highest_point);
        let jet_direction = jet_sequence[jet_index].clone();
        let mut next_move = Move::Lateral(jet_direction.clone());

        loop {
            match next_move {
                Move::Lateral(direction) => {
                    move_block(&mut block, &area, Move::Lateral(direction));
                    next_move = Move::Down;
                }
                Move::Down => {
                    let is_valid = move_block(&mut block, &area, Move::Down);

                    update_jet_index(&mut jet_index, &jet_sequence);
                    let jet_direction = jet_sequence[jet_index].clone();

                    next_move = Move::Lateral(jet_direction);

                    if !is_valid {
                        highest_point = update_area(&block, &mut area, highest_point);
                        record_jet_index(jet_index, highest_point, &mut jet_record);
                        break;
                    }
                }
            }
        }
    }
    print_area(&area, &jet_record);

    println!("Highest point: {}", highest_point);
    */

    /*
    let (pattern, start, end, blocks_before) = identify_pattern(&jet_record);
    let length = pattern.len();
    */
    let start = 3225;
    let end = 5959;
    let length = 1725;
    let blocks_before = 2040;
    println!("Block count: {}", BLOCK_COUNT);

    println!("Pattern start: {}", start);
    println!("Pattern end: {}", end);
    println!("Pattern length: {}", length);
    println!("Blocks before pattern: {}", blocks_before);
    println!("Repeat: {}", (BLOCK_COUNT - blocks_before) / length);

    if length > 0 {
        let modulo = (BLOCK_COUNT - blocks_before) % length;
        let calculation = start + (BLOCK_COUNT - blocks_before - modulo) / length * (end - start);
        println!("Calculation: {}", calculation);
        println!("Calculation: {}", calculation + 2060);
        // 1584927536223 too low
        // 1584927536247 ???
        // 1584927538283 too high
        // 1_000_000_000_000 % 1725 =  1285
        // if we add 1285 blocks to any start + pattern * x, the difference is 51763 - 49703 = 2060
    }
}
