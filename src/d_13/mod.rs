pub mod input;
use std::cmp;

/*
const EXAMPLE: &str = "
[[2]]
[[6]]
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

const EXAMPLE_2: &str = "
[[1],[2,3,4]]
[[1],4]
";
*/

#[derive(Debug, Clone)]
struct N {
    v: i32,
    c: Vec<N>,
}

#[derive(Debug)]
enum Result {
    Equal,
    Smaller,
    Larger,
}

fn get_input<'a>() -> Vec<&'a str> {
    input::INPUT
        .lines()
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>()
}

fn get_children_string(s: &str) -> Vec<&str> {
    let mut children = Vec::new();
    let mut start = 0;
    let mut end: usize;
    let mut depth = 0;

    for (i, c) in s.chars().enumerate() {
        if c == '[' {
            depth += 1;
        } else if c == ']' {
            depth -= 1;
        }

        if depth == 0 && c == ',' {
            end = i;
            children.push(&s[start..end]);
            start = end + 1;
        }
    }

    children.push(&s[start..]);

    children
}

fn parse_node(str: &str) -> N {
    let mut str = str.clone();

    if str.starts_with("[") {
        str = &str[1..str.len() - 1];
    } else {
        return N {
            v: str.parse().unwrap_or(-1),
            c: vec![],
        };
    }

    let children_strings = get_children_string(str);

    if children_strings.len() == 1 && children_strings[0].len() == 0 {
        return N { v: 0, c: vec![] };
    }

    let children = children_strings
        .iter()
        .map(|s| parse_node(s))
        .collect::<Vec<N>>();

    return N { v: 0, c: children };
}

fn get_packets(input: &Vec<&str>) -> Vec<N> {
    input.iter().map(|a| parse_node(*a)).collect::<Vec<N>>()
}

fn is_left_smaller(l: &N, r: &N) -> Result {
    if l.c.len() == 0 && r.c.len() == 0 {
        if l.v < r.v {
            return Result::Smaller;
        } else if l.v > r.v {
            return Result::Larger;
        } else {
            return Result::Equal;
        }
    }

    let mut left = l.clone();
    let mut right = r.clone();

    if l.c.len() == 0 {
        left.c = vec![l.clone()];
    }

    if r.c.len() == 0 {
        right.c = vec![r.clone()];
    }

    let size = cmp::min(left.c.len(), right.c.len());

    for i in 0..size {
        let left_child = left.c.get(i).unwrap();
        let right_child = right.c.get(i).unwrap();

        let result = is_left_smaller(left_child, right_child);
        match result {
            Result::Smaller => return Result::Smaller,
            Result::Larger => return Result::Larger,
            Result::Equal => continue,
        }
    }

    if left.c.len() < right.c.len() {
        return Result::Smaller;
    } else if left.c.len() > right.c.len() {
        return Result::Larger;
    }

    return Result::Equal;
}

pub fn main() {
    let input = get_input();
    let packets = get_packets(&input);
    let mut packets_with_index = packets
        .iter()
        .enumerate()
        .map(|(i, p)| (i, p))
        .collect::<Vec<(usize, &N)>>();

    packets_with_index.sort_by(|(_, a), (_, b)| {
        let result = is_left_smaller(a, b);
        match result {
            Result::Smaller => cmp::Ordering::Less,
            Result::Larger => cmp::Ordering::Greater,
            Result::Equal => cmp::Ordering::Equal,
        }
    });

    let mut result = 1;
    for (j, (i, _)) in packets_with_index.iter().enumerate() {
        if *i <= 1 {
            println!("{}: {}", j + 1, i);
            result *= j + 1;
        }
    }

    println!("Result: {}", result);
}
