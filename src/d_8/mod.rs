pub mod input;

/*
const EXAMPLE_INPUT: &str = "
30373
25512
65332
33549
35390
";
*/

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn explore(dir: Direction, height: u32, rows: &Vec<Vec<u32>>, r: usize, c: usize) -> usize {
    let r_len = rows.len();
    let c_len = rows[0].len();
    let mut farest = 0;

    let (start, end) = match dir {
        Direction::Up => (0, r + 1),
        Direction::Down => (r, r_len),
        Direction::Left => (0, c + 1),
        Direction::Right => (c, c_len),
    };

    for i in start..end {
        if i == start {
            continue;
        }

        let (x, y) = match dir {
            Direction::Up => (r - i, c),
            Direction::Down => (i, c),
            Direction::Left => (r, c - i),
            Direction::Right => (r, i),
        };

        farest += 1;

        if height <= rows[x][y] {
            break;
        }
    }
    return farest;
}

fn get_rows() -> Vec<Vec<u32>> {
    return input::INPUT
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();
}

pub fn main() {
    let mut trees = [[0; 99]; 99];
    let rows = get_rows();
    rows.iter().enumerate().for_each(|(r, row)| {
        for c in 0..row.len() {
            let height = row[c];
            let top = explore(Direction::Up, height, &rows, r, c);
            let bottom = explore(Direction::Down, height, &rows, r, c);
            let left = explore(Direction::Left, height, &rows, r, c);
            let right = explore(Direction::Right, height, &rows, r, c);
            trees[r][c] += top * bottom * left * right;
        }
    });

    let max = trees.iter().flatten().max().unwrap();
    println!("{}", max);
}
