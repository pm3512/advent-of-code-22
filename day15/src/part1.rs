use std::env;
use std::fs;

pub fn solve() {
    let args: Vec<String> = env::args().collect();
    let default_filename = String::from("inputs/in1.txt");
    let in_filename = &args.get(1).unwrap_or(&default_filename);

    let mut bounds = parse_input(in_filename);
    let bounds = remove_ovelaps(&mut bounds);

    let lens = bounds.iter().map(|a| a.1 - a.0);
    let sums: i32 = lens.sum();
    println!("{sums}");
}

fn remove_ovelaps(bounds:&mut Vec<(i32, i32)>) -> Vec<(i32, i32)> {
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

fn parse_input(in_filename: &str) -> Vec<(i32, i32)> {
    const Y: i32 = 2_000_000;
    let input = fs::read_to_string(in_filename)
    .expect(&format!("Couldn't open file: {in_filename}"));

    let mut bounds: Vec<(i32, i32)> = Vec::new();
    for line in input.lines() {
        let (sensor_x, sensor_y, beacon_x, beacon_y) = parse_line(line);
        let dist = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();
        let dist_y = (sensor_y - Y).abs();
        let min_dist_x = dist - dist_y;
        if min_dist_x > 0 {
            if beacon_y == Y {
                bounds.push((-min_dist_x + sensor_x, beacon_x));
                bounds.push((beacon_x + 1, min_dist_x + sensor_x + 1));
            } else {
                bounds.push((-min_dist_x + sensor_x, min_dist_x + sensor_x + 1));
            }
        }
        
    }
    bounds
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
