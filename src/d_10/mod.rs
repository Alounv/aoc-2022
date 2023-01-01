pub mod input;

/*
const EXAMPLE_INPUT: &str = "
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
*/

fn get_instructions() -> Vec<i32> {
    let mut instructions = Vec::new();
    instructions.push(1);
    input::INPUT
        .lines()
        .filter(|l| !l.is_empty())
        .for_each(|l| {
            let parts = l.split_whitespace().collect::<Vec<&str>>();
            let op = parts[0];

            instructions.push(0);
            match op {
                "addx" => {
                    instructions.push(parts[1].parse::<i32>().unwrap());
                }
                "noop" => {}
                _ => panic!("Unknown op: {}", op),
            }
        });
    return instructions;
}

fn run_instructions(instructions: &Vec<i32>) -> Vec<i32> {
    let mut x = 0;
    let cycles = instructions
        .iter()
        .map(|i| {
            x += i;
            x
        })
        .collect::<Vec<_>>();
    return cycles;
}

fn get_screen(register: &Vec<i32>) -> Vec<char> {
    let mut screen = Vec::new();
    let mut i = 0;
    for reg in register {
        let is_lit = i == *reg || i == *reg + 1 || i == *reg - 1;
        let pixel = if is_lit { '#' } else { '.' };
        screen.push(pixel);
        i += 1;
        if i == 40 {
            i = 0
        }
    }
    return screen;
}

pub fn main() {
    let instructions = get_instructions();
    let cycles = run_instructions(&instructions);
    let screen = get_screen(&cycles);
    println!("{}", screen[0..40].iter().collect::<String>());
    println!("{}", screen[40..80].iter().collect::<String>());
    println!("{}", screen[80..120].iter().collect::<String>());
    println!("{}", screen[120..160].iter().collect::<String>());
    println!("{}", screen[160..200].iter().collect::<String>());
    println!("{}", screen[200..240].iter().collect::<String>());
}
