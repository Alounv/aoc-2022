pub mod input;

/*
const EXAMPLE_INPUT: &str = "
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

const EXAMPLE_CRATES: &str = "
ZN
MCD
P
";
*/

struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn get_instruction(line: &str) -> Instruction {
    let parts = line.split_whitespace().collect::<Vec<&str>>();
    let count = parts[1].parse().expect("Cannot parse count");
    let from = parts[3].parse().expect("Cannot parse from");
    let to = parts[5].parse().expect("Cannot parse to");
    Instruction { count, from, to }
}

fn move_crates(crates: &Vec<Vec<char>>, instruction: Instruction) -> Vec<Vec<char>> {
    let Instruction { count, from, to } = instruction;
    let mut new_crates = crates.clone();
    crates[from - 1]
        .iter()
        .rev()
        .take(count)
        .rev()
        .for_each(|c| {
            new_crates[to - 1].push(*c);
            new_crates[from - 1].pop();
        });
    new_crates
}

pub fn main() {
    let mut crates: Vec<Vec<char>> = input::CRATES
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect();

    let instructions = input::INPUT
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| get_instruction(s))
        .collect::<Vec<_>>();

    for i in instructions {
        crates = move_crates(&crates, i);
    }

    let output = crates
        .iter()
        .map(|c| c.last().expect("last"))
        .collect::<String>();

    println!("{output}");
}
