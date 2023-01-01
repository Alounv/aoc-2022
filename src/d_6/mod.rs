pub mod input;

/*
const EXAMPLE_INPUT: &str = "
mjqjpqmgbljsphdztnvjfqwrcgsmlb
bvwbjplbgvbhsrlpgdmjqwftvncz
nppdvjthqldpwncqszvftbrmjlhg
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw
";
*/

fn get_packet_start(input: &str) -> usize {
    for (i, _) in input.chars().enumerate() {
        let slice = &input[i..i + 14];
        let mut chars: Vec<char> = slice.chars().collect();
        chars.sort_unstable();
        chars.dedup();
        if chars.len() == 14 {
            return i + 14;
        }
    }

    input.len()
}

pub fn main() {
    let signals = input::INPUT
        .split("\n")
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    for signal in signals {
        let packet_start: usize = get_packet_start(signal);
        println!("{} {}", signal, packet_start);
    }
}
