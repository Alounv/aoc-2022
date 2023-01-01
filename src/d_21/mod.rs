pub mod input;
use std::collections::HashMap;

/*
const EXAMPLE: &str = "
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";
*/

#[derive(Debug, Clone)]
struct M {
    v: Option<isize>,
    o: (char, String, String),
    left: Option<isize>,
    right: Option<isize>,
}

fn get_monkeys() -> HashMap<String, M> {
    let mut monkeys = HashMap::new();

    input::INPUT
        .lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            let name = parts[0].trim_end_matches(':').to_string();
            let value = parts[1].parse::<isize>();
            let monkey = match value {
                Ok(value) => M {
                    v: Some(value),
                    o: ('=', "".to_string(), "".to_string()),
                    left: None,
                    right: None,
                },
                Err(_) => {
                    let left = parts[1].to_string();
                    let op = parts[2].chars().next().unwrap();
                    let right = parts[3].to_string();
                    M {
                        v: None,
                        o: (op, left, right),
                        left: None,
                        right: None,
                    }
                }
            };
            monkeys.insert(name, monkey);
        });
    monkeys
}

pub fn main() {
    let initial_monkeys = get_monkeys();

    let test = 3_699_945_358_500;
    for humn in 63..69 {
        let mut monkeys = initial_monkeys.clone();
        monkeys.insert(
            "humn".to_string(),
            M {
                v: Some(humn + test),
                o: ('=', "".to_string(), "".to_string()),
                left: None,
                right: None,
            },
        );

        'big: loop {
            let mut new_monkeys = monkeys.clone();

            let monkeys_without_value = &monkeys
                .iter()
                .filter(|(_, m)| match m.v {
                    Some(_) => false,
                    None => true,
                })
                .collect::<Vec<_>>();

            if monkeys_without_value.is_empty() {
                break;
            }

            for m in monkeys_without_value {
                let (name, monkey) = m;
                let operation = monkey.o.clone();
                let (op, l, r) = operation;
                let mut new_monkey = M {
                    v: monkey.v,
                    o: (op, l.clone(), r.clone()),
                    left: monkey.left,
                    right: monkey.right,
                };
                if None == monkey.left {
                    let left_monkey = monkeys.get(&l).unwrap();
                    if left_monkey.v.is_some() {
                        new_monkey.left = left_monkey.v;
                    }
                }
                if None == monkey.right {
                    let right_monkey = monkeys.get(&r).unwrap();
                    if right_monkey.v.is_some() {
                        new_monkey.right = right_monkey.v;
                    }
                }

                if None != new_monkey.left && None != new_monkey.right {
                    if *name == "root" {
                        println!(
                            "{} {}",
                            humn,
                            (new_monkey.left.unwrap() - new_monkey.right.unwrap())
                        );
                        if new_monkey.left.unwrap() == new_monkey.right.unwrap() {
                            println!(
                                "Found it: {}, humn {} {}",
                                new_monkey.left.unwrap(),
                                humn,
                                humn + test
                            );
                            break 'big;
                        }
                        break 'big;
                    } else {
                        let left = new_monkey.left.unwrap();
                        let right = new_monkey.right.unwrap();
                        let value = match op {
                            '+' => left + right,
                            '-' => left - right,
                            '*' => left * right,
                            '/' => left / right,
                            _ => panic!("Unknown operator"),
                        };
                        new_monkey.v = Some(value);

                        new_monkeys.insert(name.to_string(), new_monkey);
                    }
                }
            }

            monkeys = new_monkeys;
        }
    }
}

// PART 2 - 'fzvp' should be equal to 88521161883075
