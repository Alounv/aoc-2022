pub mod input;

/*
const EXAMPLE: &str = "
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";
*/

fn get_value(part: &str) -> i32 {
    part.split(&['=', ',', ':'][..])
        .nth(1)
        .unwrap()
        .parse::<i32>()
        .unwrap()
}

fn get_positions(l: &str) -> ((i32, i32), (i32, i32)) {
    let parts = l.split_whitespace().collect::<Vec<_>>();

    let x = get_value(parts[2]);
    let y = get_value(parts[3]);
    let s = (x, y);

    let x = get_value(parts[8]);
    let y = get_value(parts[9]);
    let b = (x, y);

    (s, b)
}

fn get_sensor_distance() -> Vec<((i32, i32), i32)> {
    let lines = input::INPUT.lines().filter(|l| !l.is_empty());
    lines
        .map(|l| get_positions(l))
        .map(|(s, b)| (s, (s.0 - b.0).abs() + (s.1 - b.1).abs()))
        .collect::<Vec<((i32, i32), i32)>>()
}

fn get_circle(ox: i32, oy: i32, r: i32) -> Vec<(i32, i32)> {
    let mut points = Vec::new();
    for x in 0..=r {
        let y = r - x;
        points.push((ox + x, oy + y));
        points.push((ox + x, oy - y));
        points.push((ox - x, oy + y));
        points.push((ox - x, oy - y));
    }
    points
}

fn get_external_circles(sensors: &Vec<((i32, i32), i32)>) -> Vec<Vec<(i32, i32)>> {
    sensors
        .iter()
        .map(|(s, d)| get_circle(s.0, s.1, *d + 1))
        .collect::<Vec<Vec<(i32, i32)>>>()
}

fn is_not_in_range(sensors: &Vec<((i32, i32), i32)>, p: &(i32, i32)) -> bool {
    for ((sx, sy), d) in sensors.iter() {
        let dist = (p.0 - sx).abs() + (p.1 - sy).abs();
        if dist <= *d {
            return false;
        }
    }
    true
}

pub fn main() {
    let sensors = get_sensor_distance();
    let ext_circles = get_external_circles(&sensors);
    let min = 0;
    let max = 4_000_000;

    for circle in ext_circles {
        for point in circle {
            if point.0 < min || point.0 > max || point.1 < min || point.1 > max {
                continue;
            }
            if is_not_in_range(&sensors, &point) {
                println!("{:?}", point);
                println!("{}", point.0 as i64 * 4_000_000 as i64 + point.1 as i64);
                return;
            }
        }
    }
}
