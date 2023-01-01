pub mod input;

/*
const EXAMPLE_INPUT: &str = "
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
*/

fn get_range_from_area(area: &str) -> (u32, u32) {
    let range = area.split('-').collect::<Vec<&str>>();
    let start = range[0]
        .parse::<u32>()
        .expect("Failed to parse start of range");
    let end = range[1]
        .parse::<u32>()
        .expect("Failed to parse end of range");
    (start, end)
}

fn get_ranges_from_pair(pair: &str) -> ((u32, u32), (u32, u32)) {
    let areas = pair.split(',').collect::<Vec<&str>>();
    let (start1, end1) = get_range_from_area(areas[0]);
    let (start2, end2) = get_range_from_area(areas[1]);
    ((start1, end1), (start2, end2))
}

fn get_is_overlap(pair: &str) -> bool {
    let ((start1, end1), (start2, end2)) = get_ranges_from_pair(pair);
    start1 <= start2 && end1 >= end2 || start2 <= start1 && end2 >= end1
}

fn get_is_partial_overlap(pair: &str) -> bool {
    let ((start1, end1), (start2, end2)) = get_ranges_from_pair(pair);
    !(end1 < start2 || end2 < start1)
}

pub fn main() {
    let pairs = input::INPUT
        .split("\n")
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    let overlaps = pairs.iter().filter(|pair| get_is_overlap(pair)).count();

    println!("Part 1: {overlaps}");

    let overlaps = pairs
        .iter()
        .filter(|pair| get_is_partial_overlap(pair))
        .count();

    println!("Part 2: {overlaps}");
}
