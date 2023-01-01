pub mod input;

const KEY: isize = 811589153;

/*
const EXAMPLE: &str = "
1
2
-3
3
-2
0
4
";
*/

#[derive(Debug, Clone)]
struct Point {
    order: usize,
    value: isize,
}

fn get_input() -> Vec<Point> {
    let input = input::INPUT;
    //let input = EXAMPLE;
    input
        .lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(i, l)| {
            let value = l.parse::<isize>().unwrap() * KEY;
            let order = i;
            Point { order, value }
        })
        .collect()
}

fn move_element(arr: &mut Vec<Point>, old_index: usize, new_index: usize) {
    if old_index < new_index {
        arr[old_index..=new_index].rotate_left(1);
    } else {
        arr[new_index..=old_index].rotate_right(1);
    }
}

pub fn main() {
    let initial_input = get_input();
    let input_len = initial_input.len();
    let mut input = get_input()
        .iter()
        .map(|p| Point {
            order: p.order,
            value: p.value % (input_len as isize - 1),
        })
        .collect::<Vec<Point>>();

    for _ in 0..10 {
        for i in 0..input_len {
            let position = input.iter().position(|p| p.order == i).unwrap();
            let value = input[position].value;
            if value == 0 {
                continue;
            }

            let mut new_position = position as isize + value;

            if value < 0 && new_position == 0 {
                new_position = input_len as isize - 1;
            }

            if new_position < 0 {
                new_position = input_len as isize + new_position - 1;
                if new_position == -1 {
                    new_position = input_len as isize + new_position - 1;
                }
                assert!(new_position >= 0);
            }

            if new_position >= input_len as isize {
                new_position = new_position - input_len as isize + 1;
                if new_position == input_len as isize {
                    new_position = 1;
                }
                assert!(new_position < input_len as isize);
            }

            move_element(&mut input, position, new_position as usize);
        }
    }

    let input = input
        .iter()
        .map(|p| initial_input.iter().find(|i| i.order == p.order).unwrap())
        .map(|p| p.clone())
        .collect::<Vec<Point>>();
    println!(
        "{:?}",
        input.iter().map(|p| p.value).collect::<Vec<isize>>()
    );

    let zero = input.iter().position(|p| p.value == 0).unwrap();
    let xi = (zero + 1000) % input_len;
    let yi = (zero + 2000) % input_len;
    let zi = (zero + 3000) % input_len;

    let x = input[xi].value;
    let y = input[yi].value;
    let z = input[zi].value;

    println!("{} {} {}", x, y, z);
    println!("{}", x + y + z);
}

// 14262 not right
