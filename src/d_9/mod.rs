pub mod input;

/*
const EXAMPLE_INPUT: &str = "
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
*/

#[derive(Debug, Clone, Copy)]
enum Dir {
    U,
    D,
    L,
    R,
}

fn get_moves() -> Vec<Dir> {
    let lines = input::INPUT
        .split("\n")
        .filter(|l| l != &"")
        .collect::<Vec<&str>>();

    let moves = lines
        .iter()
        .map(|l| {
            let parts = l.split(" ").collect::<Vec<&str>>();
            let dir = match parts[0].chars().next().unwrap() {
                'U' => Dir::U,
                'D' => Dir::D,
                'L' => Dir::L,
                'R' => Dir::R,
                _ => panic!("Unknown direction"),
            };

            let count = parts[1].parse().unwrap();
            let steps = (0..count).map(|_| dir).collect::<Vec<Dir>>();

            return steps;
        })
        .flatten()
        .collect::<Vec<Dir>>();

    return moves;
}

fn get_head_positions(moves: &Vec<Dir>) -> Vec<(i32, i32)> {
    let mut positions = Vec::new();
    positions.push((0, 0));
    moves.iter().for_each(|m| {
        let (mx, my) = match m {
            Dir::U => (0, 1),
            Dir::D => (0, -1),
            Dir::L => (-1, 0),
            Dir::R => (1, 0),
        };
        let (x, y) = positions.last().unwrap_or(&(0, 0));
        let new_pos = (x + mx, y + my);
        positions.push(new_pos);
    });
    return positions;
}

fn get_following_position(head_positions: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut positions = Vec::new();

    head_positions.iter().for_each(|(hx, hy)| {
        let (tx, ty) = positions.last().unwrap_or(&(0, 0));
        let new_pos;
        if hx - tx > 1 && hy - ty > 1 {
            new_pos = (tx + 1, ty + 1);
        } else if hx - tx > 1 && ty - hy > 1 {
            new_pos = (tx + 1, ty - 1);
        } else if tx - hx > 1 && hy - ty > 1 {
            new_pos = (tx - 1, ty + 1);
        } else if tx - hx > 1 && ty - hy > 1 {
            new_pos = (tx - 1, ty - 1);
        } else if hx - tx > 1 {
            new_pos = (tx + 1, *hy);
        } else if hx - tx < -1 {
            new_pos = (tx - 1, *hy);
        } else if hy - ty > 1 {
            new_pos = (*hx, ty + 1);
        } else if hy - ty < -1 {
            new_pos = (*hx, ty - 1);
        } else {
            new_pos = (*tx, *ty);
        }
        positions.push(new_pos);
    });
    return positions;
}

pub fn main() {
    let moves = get_moves();
    let hp = get_head_positions(&moves);
    let mut positions = Vec::new();
    positions.push(hp);
    for i in 1..10 {
        positions.push(get_following_position(&positions[i - 1]));
    }

    let last = positions.last().unwrap();
    let tail_positions_count = last.iter().collect::<std::collections::HashSet<_>>().len();
    println!(" {}", tail_positions_count);
}
