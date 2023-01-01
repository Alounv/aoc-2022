pub mod input;

/*
const EXAMPLE_INPUT: &str = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
*/

const CHARS: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn get_char_priority(c: char) -> usize {
    return 1 + CHARS.iter().position(|&x| x == c).unwrap();
}

fn get_misplaced_items_priorities(bags: &Vec<&str>) -> usize {
    let mut score = 0;

    for (_, bag) in bags.iter().enumerate() {
        let bag_size = bag.len();
        if bag_size == 0 {
            continue;
        }

        let comp_one = &bag[..bag_size / 2];
        let comp_two = &bag[bag_size / 2..];

        let first_common_item = comp_one.chars().find(|c| comp_two.contains(*c)).unwrap();
        let char_score = get_char_priority(first_common_item);

        score += char_score;
    }

    return score;
}

fn get_badge_priorities(bags: &Vec<&str>) -> usize {
    let mut score = 0;
    let groups_count = bags.len() / 3;

    for i in 0..groups_count {
        let bag1 = bags[i * 3];
        let bag2 = bags[i * 3 + 1];
        let bag3 = bags[i * 3 + 2];

        for (_, c) in bag1.chars().enumerate() {
            if bag2.contains(c) && bag3.contains(c) {
                score += get_char_priority(c);
                break;
            }
        }
    }

    return score;
}

pub fn main() {
    let bags = input::ITEMS
        .split("\n")
        .skip_while(|s| s.is_empty())
        .collect::<Vec<&str>>();

    let score = get_misplaced_items_priorities(&bags);
    println!("Common items: {}", score);

    let score2 = get_badge_priorities(&bags);
    println!("Badges: {}", score2);
}
