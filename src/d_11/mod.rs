pub mod input;

struct Monkey {
    inspect: Box<dyn Fn(u64) -> u64>,
    get_next_monkey: Box<dyn Fn(u64) -> usize>,
}

#[derive(Debug, Clone)]
struct MonkeyStates {
    items: Vec<u64>,
    inspected_items_count: u64,
}

/*
fn get_example_monkeys() -> Vec<Monkey> {
    return vec![
        Monkey {
            inspect: Box::new(|old| old * 19),
            get_next_monkey: Box::new(|x| if x % 23 == 0 { 2 } else { 3 }),
        },
        Monkey {
            inspect: Box::new(|old| old + 6),
            get_next_monkey: Box::new(|x| if x % 19 == 0 { 2 } else { 0 }),
        },
        Monkey {
            inspect: Box::new(|old| old * old),
            get_next_monkey: Box::new(|x| if x % 13 == 0 { 1 } else { 3 }),
        },
        Monkey {
            inspect: Box::new(|old| old + 3),
            get_next_monkey: Box::new(|x| if x % 17 == 0 { 0 } else { 1 }),
        },
    ];
}
*/

/*
fn example_worry_reducer(worry: u64) -> u64 {
    let reducer = 23 * 19 * 13 * 17;
    return worry % reducer;
}
*/

/*
fn get_example_monkey_states() -> Vec<MonkeyStates> {
    return vec![
        MonkeyStates {
            items: vec![79, 98],
            inspected_items_count: 0,
        },
        MonkeyStates {
            items: vec![54, 65, 75, 74],
            inspected_items_count: 0,
        },
        MonkeyStates {
            items: vec![79, 60, 97],
            inspected_items_count: 0,
        },
        MonkeyStates {
            items: vec![74],
            inspected_items_count: 0,
        },
    ];
}
*/

fn get_monkeys() -> Vec<Monkey> {
    return vec![
        Monkey {
            inspect: Box::new(|old| old * 2),
            get_next_monkey: Box::new(|x| if x % 5 == 0 { 6 } else { 1 }),
        },
        Monkey {
            inspect: Box::new(|old| old * 13),
            get_next_monkey: Box::new(|x| if x % 2 == 0 { 2 } else { 6 }),
        },
        Monkey {
            inspect: Box::new(|old| old + 5),
            get_next_monkey: Box::new(|x| if x % 19 == 0 { 7 } else { 5 }),
        },
        Monkey {
            inspect: Box::new(|old| old + 6),
            get_next_monkey: Box::new(|x| if x % 7 == 0 { 0 } else { 4 }),
        },
        Monkey {
            inspect: Box::new(|old| old + 1),
            get_next_monkey: Box::new(|x| if x % 17 == 0 { 0 } else { 1 }),
        },
        Monkey {
            inspect: Box::new(|old| old + 4),
            get_next_monkey: Box::new(|x| if x % 13 == 0 { 4 } else { 3 }),
        },
        Monkey {
            inspect: Box::new(|old| old + 2),
            get_next_monkey: Box::new(|x| if x % 3 == 0 { 2 } else { 7 }),
        },
        Monkey {
            inspect: Box::new(|old| old * old),
            get_next_monkey: Box::new(|x| if x % 11 == 0 { 3 } else { 5 }),
        },
    ];
}

fn worry_reducer(worry: u64) -> u64 {
    let reducer = 5 * 2 * 19 * 7 * 17 * 13 * 3 * 11;
    return worry % reducer;
}

fn get_monkey_states() -> Vec<MonkeyStates> {
    return vec![
        MonkeyStates {
            items: vec![98, 89, 52],
            inspected_items_count: 0,
        },
        MonkeyStates {
            items: vec![57, 95, 80, 92, 57, 78],
            inspected_items_count: 0,
        },
        MonkeyStates {
            items: vec![82, 74, 97, 75, 51, 92, 83],
            inspected_items_count: 0,
        },
        MonkeyStates {
            items: vec![97, 88, 51, 68, 76],
            inspected_items_count: 0,
        },
        MonkeyStates {
            items: vec![63],
            inspected_items_count: 0,
        },
        MonkeyStates {
            items: vec![94, 91, 51, 63],
            inspected_items_count: 0,
        },
        MonkeyStates {
            items: vec![61, 54, 94, 71, 74, 68, 98, 83],
            inspected_items_count: 0,
        },
        MonkeyStates {
            items: vec![90, 56],
            inspected_items_count: 0,
        },
    ];
}

fn get_truc<'a>(
    monkeys: &'a Vec<Monkey>,
    initial_states: &'a Vec<MonkeyStates>,
) -> Vec<MonkeyStates> {
    let mut states = initial_states.clone();

    for _round in 0..10_000 {
        for (current_monkey_index, monkey) in monkeys.iter().enumerate() {
            let current_monkey_items = states[current_monkey_index].items.clone();
            current_monkey_items.iter().for_each(|worry| {
                let worry = (monkey.inspect)(*worry);
                let worry = worry_reducer(worry);
                let next_monkey_index: usize = (monkey.get_next_monkey)(worry);
                states[next_monkey_index].items.push(worry);
                states[current_monkey_index].items.remove(0);
                states[current_monkey_index].inspected_items_count += 1;
            });
        }
    }

    return states;
}

pub fn main() {
    let monkeys = get_monkeys();
    let initial_states = get_monkey_states();
    let states = get_truc(&monkeys, &initial_states);
    println!("{:?}", states);
    let mut counts = states
        .iter()
        .map(|state| state.inspected_items_count)
        .collect::<Vec<u64>>();
    println!("{:?}", counts);
    counts.sort();
    counts.reverse();
    let top_2 = counts.iter().take(2).product::<u64>();
    println!("Top 2: {}", top_2);
}
