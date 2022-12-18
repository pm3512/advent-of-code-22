use std::env;
use std::fs;
use std::collections::HashSet;

const MAX_COORD: i32 = 21;
#[derive(Hash, PartialEq, Eq)]
enum Direction {
    Right,
    Up,
    Forward
}

fn dfs(x: i32, y: i32, z: i32, accessible: &mut HashSet<(i32, i32, i32)>, blocks: &HashSet<(i32, i32, i32)>) {
    accessible.insert((x, y, z));
    for (dx, dy, dz) in [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1)
    ] {
        let x = x + dx;
        let y = y + dy;
        let z = z + dz;
        if !blocks.contains(&(x, y, z)) &&
            -1 <= x.min(y.min(z)) &&
            x.max(y.max(z)) <= MAX_COORD + 1 &&
            !accessible.contains(&(x, y, z)) {
            dfs(x, y, z, accessible, blocks);
        }
    }
}

pub fn solve() {
    let args: Vec<String> = env::args().collect();
    let default_filename = String::from("inputs/in1.txt");
    let in_filename = &args.get(1).unwrap_or(&default_filename);

    let mut edges: HashSet<(i32, i32, i32, Direction)> = HashSet::new();
    let mut blocks: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut accessible: HashSet<(i32, i32, i32)> = HashSet::new();


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

    dfs(-1, -1, -1, &mut accessible, &blocks);

    let mut count = 0;
    for (x, y, z, dir) in edges {
        if accessible.contains(&(x, y, z)) || match dir {
            Direction::Right => accessible.contains(&(x + 1, y, z)),
            Direction::Up => accessible.contains(&(x, y + 1, z)),
            Direction::Forward => accessible.contains(&(x, y, z + 1)),
        } {
            count += 1;
        }
    }
    println!("{count}");
}