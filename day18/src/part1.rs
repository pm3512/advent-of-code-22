use std::env;
use std::fs;
use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq)]
enum Direction {
    Right,
    Up,
    Forward
}

pub fn solve() {
    let args: Vec<String> = env::args().collect();
    let default_filename = String::from("inputs/in1.txt");
    let in_filename = &args.get(1).unwrap_or(&default_filename);

    let mut edges: HashSet<(i32, i32, i32, Direction)> = HashSet::new();
    let mut blocks: HashSet<(i32, i32, i32)> = HashSet::new();


    let input = fs::read_to_string(in_filename).unwrap();
    for line in input.lines() {
        let coords: Vec<&str> = line.trim().split(",").collect();
        let x: i32 = coords.get(0).unwrap().parse().unwrap();
        let y: i32 = coords.get(1).unwrap().parse().unwrap();
        let z: i32 = coords.get(2).unwrap().parse().unwrap();

        blocks.insert((x, y, z));

        for d in [0, 1] {
            if !edges.contains(&(x - d, y, z, Direction::Right)) {
                edges.insert((x - d, y, z, Direction::Right));
            } else {
                edges.remove(&(x - d, y, z, Direction::Right));
            }
            if !edges.contains(&(x, y - d, z, Direction::Up)) {
                edges.insert((x, y - d, z, Direction::Up));
            } else {
                edges.remove(&(x, y - d, z, Direction::Up));
            }
            if !edges.contains(&(x, y, z - d, Direction::Forward)) {
                edges.insert((x, y, z - d, Direction::Forward));
            } else {
                edges.remove(&(x, y, z - d, Direction::Forward));
            }
        }
    }

    println!("{}", edges.len());
}