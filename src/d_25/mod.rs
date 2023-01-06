pub mod input;
//use colored::Colorize;
use std::collections::HashMap;

/*
const EXAMPLE: &str = "
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";
*/

fn get_input() -> Vec<Vec<char>> {
    //let input = EXAMPLE;
    let input = input::I;
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect()
}

const SIGNS: [(char, i8); 5] = [('0', 0), ('1', 1), ('2', 2), ('-', -1), ('=', -2)];

fn get_number(num: &Vec<char>) -> isize {
    let signs = SIGNS.iter().cloned().collect::<HashMap<char, i8>>();
    num.iter().rev().enumerate().fold(0, |acc, (i, c)| {
        let value = signs.get(c).unwrap();
        acc + *value as isize * 5_isize.pow(i as u32) as isize
    })
}

fn get_numbers(input: &Vec<Vec<char>>) -> Vec<isize> {
    input.iter().map(get_number).collect()
}

fn convert_to_snafu(number: &isize) -> String {
    let mut rest = *number;
    let signs: HashMap<i8, char> = SIGNS.iter().map(|(s, n)| (*n, *s)).collect();

    let mut base = vec![];
    while rest > 0 {
        base.push((rest % 5) as i8);
        rest /= 5;
    }

    let mut snafu = base
        .iter()
        .enumerate()
        .map(|(i, n)| (i, *n))
        .collect::<HashMap<usize, i8>>();

    for i in 0..snafu.len() {
        let n = snafu.get(&i).unwrap();
        if n > &2 {
            snafu.insert(i, n - 5);
            snafu.entry(i + 1).and_modify(|e| *e += 1).or_insert(1);
            continue;
        } else {
            snafu.insert(i, *n);
        }
    }

    let mut result = vec![];
    for i in 0..snafu.len() {
        let n = snafu.get(&i).unwrap();
        result.push(*signs.get(n).unwrap());
    }

    result.reverse();

    result.into_iter().collect()
}

pub fn main() {
    let input = get_input();
    let numbers = get_numbers(&input);

    let sum = numbers.iter().sum::<isize>();
    println!("sum: {:?}", sum);

    let snafu = convert_to_snafu(&sum);
    println!("snafu sum: {:?}", snafu);

    let confirm = get_number(&snafu.chars().collect());
    assert_eq!(sum, confirm);
}
