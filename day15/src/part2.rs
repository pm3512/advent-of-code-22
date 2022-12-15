use std::env;
use std::fs;

pub fn solve() {
    let args: Vec<String> = env::args().collect();
    let default_filename = String::from("inputs/in1.txt");
    let in_filename = &args.get(1).unwrap_or(&default_filename);

    parse_input(in_filename);
}

fn remove_ovelaps(bounds:&mut Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    if bounds.is_empty() {
        return Vec::new();
    }
    bounds.sort_by(|a, b| a.0.cmp(&b.0));
    let (mut start, mut end) = bounds.get(0).unwrap();

    let mut new_bounds: Vec<(i32, i32)> = Vec::new();

    for (cur_start, cur_end) in &bounds[1..] {
        if cur_start > &end {
            new_bounds.push((start, end));
            start = *cur_start;
            end = *cur_end;
        } else {
            end = end.max(*cur_end);
        }
    }
    new_bounds.push((start, end));

    new_bounds
}

fn parse_input(in_filename: &str) {
    const YMAX: i32 = 4_000_000;
    const XMAX: i32 = 4_000_000;
    let input = fs::read_to_string(in_filename)
    .expect(&format!("Couldn't open file: {in_filename}"));

    let coords: Vec<(i32, i32, i32, i32)> = input.lines().map(|line| parse_line(line)).collect();

    for y in 0..=YMAX {
        let mut bounds: Vec<(i32, i32)> = Vec::new();
        for (sensor_x, sensor_y, beacon_x, beacon_y) in &coords {
            let dist = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();
            let dist_y = (sensor_y - y).abs();
            let min_dist_x = dist - dist_y;
            if min_dist_x > 0 {
                bounds.push((-min_dist_x + sensor_x, min_dist_x + sensor_x + 1));
            }
        }

        let bounds = remove_ovelaps(&mut bounds);
        for bound in bounds {
            let freq = if bound.0 > 0 {
                i64::from(bound.0 - 1) * 4_000_000i64 + i64::from(y)
            } else if bound.1 <= XMAX {
                i64::from(bound.1) * 4_000_000i64 + i64::from(y)
            } else {
                continue;
            };
            println!("{freq}");
            return;
        }
    };
}

fn parse_line(line: &str) -> (i32, i32, i32, i32) {
    let input: Vec<&str> = line.trim().split(" ").collect();

    let sensor_x = input.get(2) .unwrap();
    let sensor_x: i32 = sensor_x[2..(sensor_x.len() - 1)].parse() .unwrap();

    let sensor_y = input.get(3) .unwrap();
    let sensor_y: i32 = sensor_y[2..(sensor_y.len() - 1)].parse() .unwrap();

    let beacon_x = input.get(8) .unwrap();
    let beacon_x: i32 = beacon_x[2..(beacon_x.len() - 1)].parse() .unwrap();

    let beacon_y = input.get(9) .unwrap();
    let beacon_y: i32 = beacon_y[2..].parse() .unwrap();

    (sensor_x, sensor_y, beacon_x, beacon_y)
}
